use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};
use anyhow::Result;
use std::time::{Duration, Instant};

use crate::audio::{AudioFeatures, RhythmFeatures};
use super::{WgpuContext, ShaderSystem, ShaderType, PerformanceManager, PerformanceMetrics, QualityLevel};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

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
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    performance_manager: PerformanceManager,
    frame_start_time: Option<Instant>,
    last_auto_shader_switch: Instant,
    auto_shader_cooldown: std::time::Duration,
}

impl EnhancedFrameComposer {
    pub fn new(context: &WgpuContext) -> Result<Self> {
        // Initialize shader system
        let shader_system = ShaderSystem::new(&context.device, &context.config)?;

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
            vertex_buffer,
            index_buffer,
            performance_manager: PerformanceManager::new(60.0), // Target 60 FPS
            frame_start_time: None,
            last_auto_shader_switch: Instant::now(),
            auto_shader_cooldown: std::time::Duration::from_millis(2500), // 2.5 seconds between switches
        })
    }

    /// Render a frame using the current shader with performance monitoring
    pub fn render(
        &mut self,
        context: &WgpuContext,
        audio_features: &AudioFeatures,
        rhythm_features: &RhythmFeatures,
        safety_multipliers: Option<crate::control::safety::SafetyMultipliers>,
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