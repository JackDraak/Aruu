use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};
use anyhow::Result;
use std::time::{Duration, Instant};

use crate::audio::{AudioFeatures, RhythmFeatures};
use super::{WgpuContext, ShaderSystem, ShaderType, PerformanceManager, PerformanceMetrics, QualityLevel, OverlaySystem};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    #[allow(dead_code)] // Reserved for future vertex buffer layouts
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    #[allow(dead_code)] // Reserved for future vertex buffer layouts
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

/// Enhanced frame composer using the new shader system architecture
pub struct EnhancedFrameComposer {
    shader_system: ShaderSystem,
    overlay_system: OverlaySystem,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    performance_manager: PerformanceManager,
    frame_start_time: Option<Instant>,
    last_auto_shader_switch: Instant,
    auto_shader_cooldown: std::time::Duration,
    // Overlay state
    show_debug_overlay: bool,
    show_control_panel: bool,
    mouse_position: (f32, f32),
    mouse_pressed: bool,
}

impl EnhancedFrameComposer {
    pub fn new(context: &WgpuContext) -> Result<Self> {
        // Initialize shader system
        let shader_system = ShaderSystem::new(&context.device, &context.config)?;

        // Initialize overlay system
        let overlay_system = OverlaySystem::new(context)?;

        // Create vertex buffer
        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Enhanced Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // Create index buffer
        let index_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Enhanced Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

        Ok(Self {
            shader_system,
            overlay_system,
            vertex_buffer,
            index_buffer,
            performance_manager: PerformanceManager::new(60.0), // Target 60 FPS
            frame_start_time: None,
            last_auto_shader_switch: Instant::now(),
            auto_shader_cooldown: std::time::Duration::from_millis(2500), // 2.5 seconds between switches
            // Overlay state defaults
            show_debug_overlay: true,  // Show debug overlay by default
            show_control_panel: true,  // Show control panel by default
            mouse_position: (0.0, 0.0),
            mouse_pressed: false,
        })
    }

    /// Render a frame using the current shader with performance monitoring
    pub fn render(
        &mut self,
        context: &WgpuContext,
        audio_features: &AudioFeatures,
        rhythm_features: &RhythmFeatures,
        safety_multipliers: Option<crate::control::safety::SafetyMultipliers>,
        volume: f32,
    ) -> Result<()> {
        // Check for emergency stop - if active, render black screen instead of shaders
        if let Some(ref multipliers) = safety_multipliers {
            if multipliers.beat_intensity == 0.0 && multipliers.brightness_range <= 0.1 {
                // Emergency stop is active - render solid black screen
                return self.render_emergency_blackout(context);
            }
        }

        // Start frame timing
        let frame_start = Instant::now();
        self.frame_start_time = Some(frame_start);

        // Update shader system (handles transitions, etc.)
        self.shader_system.update(&context.device, &context.config)?;

        // Get surface texture
        let output = context.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Render using shader system with performance awareness
        let current_quality = self.performance_manager.current_quality();
        self.shader_system.render_with_quality(
            &context.device,
            &context.queue,
            &view,
            &self.vertex_buffer,
            &self.index_buffer,
            INDICES.len() as u32,
            audio_features,
            rhythm_features,
            current_quality,
            safety_multipliers,
        )?;

        // Update overlay system state
        self.overlay_system.update(
            self.mouse_position,
            self.mouse_pressed,
            self.show_debug_overlay,
            self.show_control_panel,
        );

        // Create overlay uniforms with current state
        let overlay_uniforms = self.create_overlay_uniforms(
            audio_features,
            rhythm_features,
            safety_multipliers.as_ref(),
            &context,
            volume,
        );

        // Render overlay shaders on top of main visualization
        if let Err(e) = self.overlay_system.render(context, &view, &overlay_uniforms) {
            eprintln!("Overlay rendering error: {}", e);
            // Continue without overlays rather than crash
        }

        output.present();

        // Update performance metrics
        let frame_time = frame_start.elapsed();
        let metrics = PerformanceMetrics {
            frame_time,
            cpu_time: frame_time, // Simplified - in real app would measure separately
            gpu_time: Duration::from_secs_f32(frame_time.as_secs_f32() * 0.7), // Estimate GPU portion
            fps: 1.0 / frame_time.as_secs_f32(),
            dropped_frames: if frame_time.as_millis() > 20 { 1 } else { 0 },
            memory_usage_mb: 150.0, // Estimate
        };

        let quality_changed = self.performance_manager.update(metrics);

        // Log performance adjustments
        if quality_changed {
            println!("📊 {}", self.performance_manager.performance_report());
        }

        Ok(())
    }

    /// Switch to a different shader mode
    pub fn set_shader(&mut self, shader_type: ShaderType, context: &WgpuContext) -> Result<()> {
        self.shader_system.set_shader(shader_type, &context.device, &context.config)
    }

    /// Set shader immediately without transition animation (for manual user input)
    pub fn set_shader_immediately(&mut self, shader_type: ShaderType, context: &WgpuContext) -> Result<()> {
        self.shader_system.set_shader_immediately(shader_type, &context.device, &context.config)
    }

    /// Get the currently active shader
    pub fn current_shader(&self) -> ShaderType {
        self.shader_system.current_shader()
    }

    /// Get list of available shaders
    pub fn available_shaders(&self) -> Vec<ShaderType> {
        self.shader_system.available_shaders()
    }

    /// Check if shader is currently transitioning
    pub fn is_transitioning(&self) -> bool {
        self.shader_system.is_transitioning()
    }

    /// Cycle to the next available shader
    pub fn next_shader(&mut self, context: &WgpuContext) -> Result<()> {
        let available = self.available_shaders();
        if available.is_empty() {
            return Ok(()); // No shaders available
        }

        let current = self.current_shader();
        let current_index = available.iter().position(|&s| s == current).unwrap_or(0);
        let next_index = (current_index + 1) % available.len();
        let next_shader = available[next_index];

        println!("🎨 Cycling to shader: {} -> {}", current.name(), next_shader.name());
        self.set_shader(next_shader, context)
    }

    /// Set shader based on audio characteristics (intelligent selection)
    pub fn auto_select_shader(&mut self,
                             context: &WgpuContext,
                             audio_features: &AudioFeatures,
                             rhythm_features: &RhythmFeatures) -> Result<()> {
        let current = self.current_shader();

        // Intelligent shader selection based on audio characteristics
        let recommended_shader = self.analyze_audio_for_shader(audio_features, rhythm_features);

        if recommended_shader != current {
            // Check cooldown to prevent rapid switching and console spam
            let now = Instant::now();
            let time_since_last_switch = now.duration_since(self.last_auto_shader_switch);

            if time_since_last_switch >= self.auto_shader_cooldown {
                println!("🤖 Auto-selecting shader: {} (based on audio analysis)", recommended_shader.name());
                self.set_shader(recommended_shader, context)?;
                self.last_auto_shader_switch = now;
            }
            // If within cooldown, silently continue with current shader
        }

        Ok(())
    }

    /// Get current performance quality level
    pub fn current_quality(&self) -> QualityLevel {
        self.performance_manager.current_quality()
    }

    /// Manually set performance quality level
    pub fn set_quality(&mut self, quality: QualityLevel) {
        self.performance_manager.set_quality(quality);
    }

    /// Get performance metrics report
    pub fn performance_report(&self) -> String {
        self.performance_manager.performance_report()
    }

    /// Get average FPS over recent history
    pub fn average_fps(&self) -> f32 {
        self.performance_manager.average_fps()
    }

    /// Render solid black screen for emergency stop
    fn render_emergency_blackout(&mut self, context: &WgpuContext) -> Result<()> {
        // Get surface texture
        let output = context.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create command encoder for clear operation
        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("emergency_blackout_encoder"),
        });

        {
            // Create render pass that clears to black
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("emergency_blackout_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), // Solid black
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            // Render pass automatically clears to black, no drawing needed
        }

        // Submit commands and present
        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn analyze_audio_for_shader(&self, audio: &AudioFeatures, rhythm: &RhythmFeatures) -> ShaderType {
        // Analyze audio characteristics to recommend optimal shader

        // High bass content -> Classic or Tunnel
        if audio.bass + audio.sub_bass > 0.7 {
            return if rhythm.tempo_confidence > 0.8 {
                ShaderType::Tunnel // Strong rhythm + bass = tunnel effect
            } else {
                ShaderType::Classic // Just bass = classic waves
            };
        }

        // High treble + onset activity -> Particle system
        if audio.treble + audio.presence > 0.6 && audio.onset_strength > 0.5 {
            return ShaderType::Particle;
        }

        // High pitch confidence + harmony -> Kaleidoscope
        if audio.pitch_confidence > 0.7 && rhythm.rhythm_stability > 0.6 {
            return ShaderType::Kaleidoscope;
        }

        // High spectral flux (dynamic changes) -> Parametric wave
        if audio.spectral_flux > 0.4 {
            return ShaderType::ParametricWave;
        }

        // High dynamic range -> Fractal
        if audio.dynamic_range > 0.6 {
            return ShaderType::Fractal;
        }

        // Default fallback
        ShaderType::Classic
    }

    /// Create overlay uniforms with current state data
    fn create_overlay_uniforms(
        &self,
        audio_features: &AudioFeatures,
        rhythm_features: &RhythmFeatures,
        safety_multipliers: Option<&crate::control::safety::SafetyMultipliers>,
        context: &WgpuContext,
        volume: f32,
    ) -> super::UniversalUniforms {
        use super::UniversalUniforms;

        // Get current shader index for UI display
        let current_shader_index = match self.current_shader() {
            ShaderType::Classic => 0.0,
            ShaderType::ParametricWave => 1.0,
            ShaderType::Plasma => 2.0,
            ShaderType::Kaleidoscope => 3.0,
            ShaderType::Tunnel => 4.0,
            ShaderType::Particle => 5.0,
            ShaderType::Fractal => 6.0,
            ShaderType::Spectralizer => 7.0,
        };

        // Calculate current FPS and performance metrics from performance manager
        let current_fps = self.average_fps();
        let frame_time = if current_fps > 0.0 { 1000.0 / current_fps } else { 16.67 };

        // Get more detailed performance info
        let quality_level = self.performance_manager.current_quality();
        let quality_index = match quality_level {
            QualityLevel::Potato => 0.0,
            QualityLevel::Low => 1.0,
            QualityLevel::Medium => 2.0,
            QualityLevel::High => 3.0,
            QualityLevel::Ultra => 4.0,
        };

        // Create uniforms with audio data and overlay-specific fields
        UniversalUniforms {
            // Copy all audio features
            sub_bass: audio_features.sub_bass,
            bass: audio_features.bass,
            mid: audio_features.mid,
            treble: audio_features.treble,
            presence: audio_features.presence,
            overall_volume: audio_features.overall_volume,
            signal_level_db: audio_features.signal_level_db,
            peak_level_db: audio_features.peak_level_db,
            dynamic_range: audio_features.dynamic_range,

            // Copy rhythm features
            beat_strength: rhythm_features.beat_strength,
            estimated_bpm: rhythm_features.estimated_bpm,
            tempo_confidence: rhythm_features.tempo_confidence,
            onset_detected: if rhythm_features.onset_detected { 1.0 } else { 0.0 },
            downbeat_detected: if rhythm_features.downbeat_detected { 1.0 } else { 0.0 },

            // Copy spectral features
            spectral_centroid: audio_features.spectral_centroid,
            spectral_rolloff: audio_features.spectral_rolloff,
            spectral_flux: audio_features.spectral_flux,
            pitch_confidence: audio_features.pitch_confidence,
            zero_crossing_rate: audio_features.zero_crossing_rate,
            onset_strength: audio_features.onset_strength,

            // Set time
            time: self.frame_start_time.map_or(0.0, |start| start.elapsed().as_secs_f32()),

            // Set overlay-specific uniforms
            mouse_x: self.mouse_position.0,
            mouse_y: self.mouse_position.1,
            mouse_pressed: if self.mouse_pressed { 1.0 } else { 0.0 },
            show_debug_overlay: if self.show_debug_overlay { 1.0 } else { 0.0 },
            show_control_panel: if self.show_control_panel { 1.0 } else { 0.0 },
            ui_volume: volume, // Actual volume from audio processor
            ui_is_playing: 1.0, // ASSUMPTION: Always playing for now
            ui_safety_level: safety_multipliers.map_or(1.0, |s| {
                // Convert safety multipliers to level (0-4 scale)
                if s.beat_intensity <= 0.1 { 0.0 } // UltraSafe
                else if s.beat_intensity <= 0.3 { 1.0 } // Safe
                else if s.beat_intensity <= 0.5 { 2.0 } // Moderate
                else if s.beat_intensity <= 0.8 { 3.0 } // Standard
                else { 4.0 } // Disabled
            }),
            ui_quality_level: quality_index,
            ui_auto_shader: 1.0, // ASSUMPTION: Auto-shader enabled by default
            ui_current_shader_index: current_shader_index,
            ui_fps: current_fps,
            ui_frame_time: frame_time,
            screen_width: context.config.width as f32,
            screen_height: context.config.height as f32,
            text_scale: 1.0,

            // Set safety multipliers
            safety_emergency_stop: safety_multipliers.map_or(1.0, |s| {
                if s.beat_intensity == 0.0 && s.brightness_range <= 0.1 { 0.0 } else { 1.0 }
            }),

            // Use defaults for other fields
            ..UniversalUniforms::default()
        }
    }

    /// Update mouse position for overlay interaction
    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }

    /// Update mouse pressed state
    pub fn update_mouse_pressed(&mut self, pressed: bool) {
        self.mouse_pressed = pressed;
    }

    /// Toggle debug overlay visibility
    pub fn toggle_debug_overlay(&mut self) {
        self.show_debug_overlay = !self.show_debug_overlay;
        println!("🔍 Debug overlay: {}", if self.show_debug_overlay { "ON" } else { "OFF" });
    }

    /// Toggle control panel visibility
    pub fn toggle_control_panel(&mut self) {
        self.show_control_panel = !self.show_control_panel;
        println!("🎛️ Control panel: {}", if self.show_control_panel { "ON" } else { "OFF" });
    }

    /// Set debug overlay visibility
    pub fn set_debug_overlay(&mut self, visible: bool) {
        self.show_debug_overlay = visible;
    }

    /// Set control panel visibility
    pub fn set_control_panel(&mut self, visible: bool) {
        self.show_control_panel = visible;
    }

    /// Handle mouse click events and return overlay events
    pub fn handle_mouse_click(&self, x: f32, y: f32) -> Vec<super::OverlayEvent> {
        self.overlay_system.handle_mouse_click(x, y)
    }

    /// Check if overlay system is visible
    pub fn has_visible_overlays(&self) -> bool {
        self.show_debug_overlay || self.show_control_panel
    }

    /// Get current mouse position
    pub fn get_mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_layout() {
        let desc = Vertex::desc();
        assert_eq!(desc.array_stride, std::mem::size_of::<Vertex>() as wgpu::BufferAddress);
        assert_eq!(desc.attributes.len(), 2);
    }

    #[test]
    fn test_geometry_data() {
        assert_eq!(VERTICES.len(), 4);
        assert_eq!(INDICES.len(), 6);

        // Verify quad covers full screen in NDC
        assert_eq!(VERTICES[0].position, [-1.0, -1.0, 0.0]); // Bottom-left
        assert_eq!(VERTICES[1].position, [1.0, -1.0, 0.0]);  // Bottom-right
        assert_eq!(VERTICES[2].position, [1.0, 1.0, 0.0]);   // Top-right
        assert_eq!(VERTICES[3].position, [-1.0, 1.0, 0.0]);  // Top-left
    }

    #[test]
    fn test_audio_analysis_for_shader() {
        use crate::audio::{AudioFeatures, RhythmFeatures};

        // Create a mock composer struct to test the audio analysis method
        struct MockComposer;

        impl MockComposer {
            fn analyze_audio_for_shader(&self, audio: &AudioFeatures, rhythm: &RhythmFeatures) -> ShaderType {
                // High bass content -> Classic or Tunnel
                if audio.bass + audio.sub_bass > 0.7 {
                    return if rhythm.tempo_confidence > 0.8 {
                        ShaderType::Tunnel
                    } else {
                        ShaderType::Classic
                    };
                }

                // High treble + onset activity -> Particle system
                if audio.treble + audio.presence > 0.6 && audio.onset_strength > 0.5 {
                    return ShaderType::Particle;
                }

                // High pitch confidence + harmony -> Kaleidoscope
                if audio.pitch_confidence > 0.7 && rhythm.rhythm_stability > 0.6 {
                    return ShaderType::Kaleidoscope;
                }

                // High spectral flux -> Parametric wave
                if audio.spectral_flux > 0.4 {
                    return ShaderType::ParametricWave;
                }

                // High dynamic range -> Fractal
                if audio.dynamic_range > 0.6 {
                    return ShaderType::Fractal;
                }

                ShaderType::Classic
            }
        }

        let composer = MockComposer;

        // Test bass-heavy music
        let bass_audio = AudioFeatures {
            bass: 0.8,
            sub_bass: 0.6,
            ..AudioFeatures::new()
        };
        let high_tempo_rhythm = RhythmFeatures {
            tempo_confidence: 0.9,
            ..RhythmFeatures::new()
        };
        assert_eq!(composer.analyze_audio_for_shader(&bass_audio, &high_tempo_rhythm), ShaderType::Tunnel);

        let low_tempo_rhythm = RhythmFeatures {
            tempo_confidence: 0.5,
            ..RhythmFeatures::new()
        };
        assert_eq!(composer.analyze_audio_for_shader(&bass_audio, &low_tempo_rhythm), ShaderType::Classic);

        // Test treble-heavy with onsets
        let treble_audio = AudioFeatures {
            treble: 0.7,
            presence: 0.5,
            onset_strength: 0.6,
            ..AudioFeatures::new()
        };
        assert_eq!(composer.analyze_audio_for_shader(&treble_audio, &high_tempo_rhythm), ShaderType::Particle);

        // Test harmonic content
        let harmonic_audio = AudioFeatures {
            pitch_confidence: 0.8,
            ..AudioFeatures::new()
        };
        let stable_rhythm = RhythmFeatures {
            rhythm_stability: 0.7,
            ..RhythmFeatures::new()
        };
        assert_eq!(composer.analyze_audio_for_shader(&harmonic_audio, &stable_rhythm), ShaderType::Kaleidoscope);

        // Test high spectral flux
        let dynamic_audio = AudioFeatures {
            spectral_flux: 0.5,
            ..AudioFeatures::new()
        };
        assert_eq!(composer.analyze_audio_for_shader(&dynamic_audio, &high_tempo_rhythm), ShaderType::ParametricWave);

        // Test high dynamic range
        let range_audio = AudioFeatures {
            dynamic_range: 0.7,
            ..AudioFeatures::new()
        };
        assert_eq!(composer.analyze_audio_for_shader(&range_audio, &high_tempo_rhythm), ShaderType::Fractal);

        // Test default case
        let default_audio = AudioFeatures::new();
        let default_rhythm = RhythmFeatures::new();
        assert_eq!(composer.analyze_audio_for_shader(&default_audio, &default_rhythm), ShaderType::Classic);
    }
}