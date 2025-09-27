use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use anyhow::{Result, anyhow};

use crate::audio::{AudioFeatures, RhythmFeatures};
use super::QualityLevel;

/// Unified uniform data structure that can support all shader types
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct UniversalUniforms {
    // 5-band frequency analysis
    pub sub_bass: f32,
    pub bass: f32,
    pub mid: f32,
    pub treble: f32,
    pub presence: f32,

    // Volume and dynamics
    pub overall_volume: f32,
    pub signal_level_db: f32,
    pub peak_level_db: f32,
    pub dynamic_range: f32,

    // Enhanced rhythm analysis
    pub beat_strength: f32,
    pub estimated_bpm: f32,
    pub tempo_confidence: f32,
    pub onset_detected: f32,        // 1.0 = true, 0.0 = false
    pub downbeat_detected: f32,     // 1.0 = true, 0.0 = false

    // Spectral characteristics
    pub spectral_centroid: f32,
    pub spectral_rolloff: f32,
    pub spectral_flux: f32,
    pub pitch_confidence: f32,
    pub zero_crossing_rate: f32,
    pub onset_strength: f32,

    // Visual controls (from existing system)
    pub time: f32,
    pub color_intensity: f32,
    pub frequency_scale: f32,
    pub saturation: f32,
    pub palette_index: f32,
    pub palette_base_hue: f32,
    pub palette_hue_range: f32,
    pub transition_blend: f32,
    pub prev_palette_index: f32,
    pub prev_palette_base_hue: f32,
    pub prev_palette_hue_range: f32,

    // Effect weights for multi-mode shaders
    pub plasma_weight: f32,
    pub kaleidoscope_weight: f32,
    pub tunnel_weight: f32,
    pub particle_weight: f32,
    pub fractal_weight: f32,
    pub spectralizer_weight: f32,

    // System parameters
    pub projection_mode: f32,      // 0.0 = 2D, 1.0 = 3D perspective
    pub smoothing_factor: f32,     // Global smoothing control
    pub resolution_x: f32,         // Screen width
    pub resolution_y: f32,         // Screen height

    // Safety multipliers for epilepsy prevention
    pub safety_beat_intensity: f32,      // Multiplier for beat-driven effects
    pub safety_onset_intensity: f32,     // Multiplier for onset-driven effects
    pub safety_color_change_rate: f32,   // Multiplier for color change rate
    pub safety_brightness_range: f32,    // Multiplier for brightness range
    pub safety_pattern_complexity: f32,  // Multiplier for pattern complexity
    pub safety_emergency_stop: f32,      // 1.0 = normal, 0.0 = emergency stop

    // Overlay system uniforms
    pub mouse_x: f32,                     // Mouse X coordinate (0.0 to 1.0)
    pub mouse_y: f32,                     // Mouse Y coordinate (0.0 to 1.0)
    pub mouse_pressed: f32,               // 1.0 = pressed, 0.0 = not pressed
    pub show_debug_overlay: f32,          // 1.0 = visible, 0.0 = hidden
    pub show_control_panel: f32,          // 1.0 = visible, 0.0 = hidden
    pub ui_volume: f32,                   // Current volume level (0.0 to 1.0)
    pub ui_is_playing: f32,               // 1.0 = playing, 0.0 = paused
    pub ui_safety_level: f32,             // Current safety level (0.0 to 4.0)
    pub ui_quality_level: f32,            // Current quality level (0.0 to 4.0)
    pub ui_auto_shader: f32,              // 1.0 = auto enabled, 0.0 = manual
    pub ui_current_shader_index: f32,     // Index of current shader (0.0 to 7.0)
    pub ui_fps: f32,                      // Current FPS for display
    pub ui_frame_time: f32,               // Current frame time in ms
    pub screen_width: f32,                // Screen width in pixels
    pub screen_height: f32,               // Screen height in pixels
    pub text_scale: f32,                  // Text scaling factor
}

impl Default for UniversalUniforms {
    fn default() -> Self {
        Self {
            // 5-band frequency analysis
            sub_bass: 0.0,
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            presence: 0.0,

            // Volume and dynamics
            overall_volume: 0.0,
            signal_level_db: -60.0,
            peak_level_db: -60.0,
            dynamic_range: 0.0,

            // Enhanced rhythm analysis
            beat_strength: 0.0,
            estimated_bpm: 120.0,
            tempo_confidence: 0.0,
            onset_detected: 0.0,
            downbeat_detected: 0.0,

            // Spectral characteristics
            spectral_centroid: 0.0,
            spectral_rolloff: 0.0,
            spectral_flux: 0.0,
            pitch_confidence: 0.0,
            zero_crossing_rate: 0.0,
            onset_strength: 0.0,

            // Visual controls
            time: 0.0,
            color_intensity: 1.0,
            frequency_scale: 1.0,
            saturation: 1.0,
            palette_index: 0.0,
            palette_base_hue: 0.0,
            palette_hue_range: 1.0,
            transition_blend: 1.0,
            prev_palette_index: 0.0,
            prev_palette_base_hue: 0.0,
            prev_palette_hue_range: 1.0,

            // Effect weights (all equal by default)
            plasma_weight: 0.2,
            kaleidoscope_weight: 0.2,
            tunnel_weight: 0.2,
            particle_weight: 0.2,
            fractal_weight: 0.1,
            spectralizer_weight: 0.1,

            // System parameters
            projection_mode: 0.0,
            smoothing_factor: 0.5,
            resolution_x: 1200.0,   // Default resolution
            resolution_y: 800.0,

            // Safety multipliers (default to Safe level)
            safety_beat_intensity: 0.3,      // Conservative beat response
            safety_onset_intensity: 0.2,     // Conservative onset response
            safety_color_change_rate: 0.4,   // Limit color changes
            safety_brightness_range: 0.5,    // Limit brightness variations
            safety_pattern_complexity: 0.5,  // Simplify patterns
            safety_emergency_stop: 1.0,      // Normal operation

            // Overlay system defaults
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_pressed: 0.0,
            show_debug_overlay: 1.0,          // Debug overlay visible by default
            show_control_panel: 1.0,          // Control panel visible by default
            ui_volume: 0.7,                   // Default volume 70%
            ui_is_playing: 0.0,               // Not playing by default
            ui_safety_level: 1.0,             // Safe level by default
            ui_quality_level: 1.0,            // High quality by default
            ui_auto_shader: 1.0,              // Auto-shader enabled by default
            ui_current_shader_index: 0.0,     // Classic shader by default
            ui_fps: 60.0,                     // Target FPS
            ui_frame_time: 16.67,             // Target frame time
            screen_width: 1200.0,             // Default screen width
            screen_height: 800.0,             // Default screen height
            text_scale: 1.0,                  // Normal text scale
        }
    }
}

/// Represents different shader types/modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Classic,
    ParametricWave,
    Plasma,
    Kaleidoscope,
    Tunnel,
    Particle,
    Fractal,
    Spectralizer,
}

impl ShaderType {
    pub fn name(&self) -> &'static str {
        match self {
            ShaderType::Classic => "Classic",
            ShaderType::ParametricWave => "Parametric Wave",
            ShaderType::Plasma => "Plasma",
            ShaderType::Kaleidoscope => "Kaleidoscope",
            ShaderType::Tunnel => "Tunnel",
            ShaderType::Particle => "Particle",
            ShaderType::Fractal => "Fractal",
            ShaderType::Spectralizer => "Spectralizer",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ShaderType::Classic => "Original Aruu visualization with radial waves",
            ShaderType::ParametricWave => "Mathematical patterns with audio-reactive parameters",
            ShaderType::Plasma => "Fluid, organic patterns driven by low frequencies",
            ShaderType::Kaleidoscope => "Symmetric patterns responding to harmonic content",
            ShaderType::Tunnel => "3D perspective effects with bass-driven depth",
            ShaderType::Particle => "Dynamic particle systems responding to transients",
            ShaderType::Fractal => "Self-similar patterns scaled by spectral characteristics",
            ShaderType::Spectralizer => "Direct frequency visualization with artistic flair",
        }
    }

    pub fn all() -> &'static [ShaderType] {
        &[
            ShaderType::Classic,
            ShaderType::ParametricWave,
            ShaderType::Plasma,
            ShaderType::Kaleidoscope,
            ShaderType::Tunnel,
            ShaderType::Particle,
            ShaderType::Fractal,
            ShaderType::Spectralizer,
        ]
    }
}

/// Metadata about a shader
#[derive(Debug, Clone)]
pub struct ShaderMetadata {
    pub shader_type: ShaderType,
    pub vertex_source: &'static str,
    pub fragment_source: &'static str,
    pub requires_3d: bool,
    pub performance_cost: u8, // 1-10 scale
}

/// Registry of available shaders
pub struct ShaderRegistry {
    shaders: HashMap<ShaderType, ShaderMetadata>,
}

impl ShaderRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            shaders: HashMap::new(),
        };

        // Register all available shaders
        registry.register_default_shaders();
        registry
    }

    fn register_default_shaders(&mut self) {
        let vertex_source = include_str!("shaders/classic.vert.wgsl");

        // Classic shader (existing implementation)
        self.register(ShaderMetadata {
            shader_type: ShaderType::Classic,
            vertex_source,
            fragment_source: include_str!("shaders/classic.frag.wgsl"),
            requires_3d: false,
            performance_cost: 3,
        });

        // Parametric wave shader
        self.register(ShaderMetadata {
            shader_type: ShaderType::ParametricWave,
            vertex_source,
            fragment_source: include_str!("shaders/parametric_wave.frag.wgsl"),
            requires_3d: false,
            performance_cost: 6,
        });

        // Plasma shader - fluid organic patterns
        self.register(ShaderMetadata {
            shader_type: ShaderType::Plasma,
            vertex_source,
            fragment_source: include_str!("shaders/plasma.frag.wgsl"),
            requires_3d: false,
            performance_cost: 7,
        });

        // Kaleidoscope shader - symmetric patterns
        self.register(ShaderMetadata {
            shader_type: ShaderType::Kaleidoscope,
            vertex_source,
            fragment_source: include_str!("shaders/kaleidoscope.frag.wgsl"),
            requires_3d: false,
            performance_cost: 5,
        });

        // Tunnel shader - 3D perspective effects
        self.register(ShaderMetadata {
            shader_type: ShaderType::Tunnel,
            vertex_source,
            fragment_source: include_str!("shaders/tunnel.frag.wgsl"),
            requires_3d: true,
            performance_cost: 6,
        });

        // Particle shader - dynamic particle systems
        self.register(ShaderMetadata {
            shader_type: ShaderType::Particle,
            vertex_source,
            fragment_source: include_str!("shaders/particle.frag.wgsl"),
            requires_3d: false,
            performance_cost: 8,
        });

        // Fractal shader - mathematical fractal patterns
        self.register(ShaderMetadata {
            shader_type: ShaderType::Fractal,
            vertex_source,
            fragment_source: include_str!("shaders/fractal.frag.wgsl"),
            requires_3d: false,
            performance_cost: 9,
        });

        // Spectralizer shader - direct frequency visualization
        self.register(ShaderMetadata {
            shader_type: ShaderType::Spectralizer,
            vertex_source,
            fragment_source: include_str!("shaders/spectralizer.frag.wgsl"),
            requires_3d: false,
            performance_cost: 7,
        });
    }

    pub fn register(&mut self, metadata: ShaderMetadata) {
        self.shaders.insert(metadata.shader_type, metadata);
    }

    pub fn get(&self, shader_type: ShaderType) -> Option<&ShaderMetadata> {
        self.shaders.get(&shader_type)
    }

    pub fn available_shaders(&self) -> Vec<ShaderType> {
        self.shaders.keys().copied().collect()
    }

    pub fn is_available(&self, shader_type: ShaderType) -> bool {
        self.shaders.contains_key(&shader_type)
    }
}

/// Manages shader transitions and blending
pub struct ShaderTransitioner {
    current_shader: ShaderType,
    target_shader: Option<ShaderType>,
    transition_progress: f32,
    transition_duration: f32,
    last_update: std::time::Instant,
}

impl ShaderTransitioner {
    pub fn new(initial_shader: ShaderType) -> Self {
        Self {
            current_shader: initial_shader,
            target_shader: None,
            transition_progress: 1.0, // Fully transitioned to current
            transition_duration: 2.0, // 2 second transitions
            last_update: std::time::Instant::now(),
        }
    }

    pub fn transition_to(&mut self, target: ShaderType) {
        if target != self.current_shader {
            self.target_shader = Some(target);
            self.transition_progress = 0.0;
            self.last_update = std::time::Instant::now();
        }
    }

    /// Immediately switch to target shader without animation (for manual user input)
    pub fn switch_immediately_to(&mut self, target: ShaderType) {
        if target != self.current_shader {
            self.current_shader = target;
            self.target_shader = None;
            self.transition_progress = 1.0;
            self.last_update = std::time::Instant::now();
        }
    }

    pub fn update(&mut self) {
        if let Some(_target) = self.target_shader {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(self.last_update).as_secs_f32();

            self.transition_progress += elapsed / self.transition_duration;

            if self.transition_progress >= 1.0 {
                // Transition complete
                self.current_shader = self.target_shader.unwrap();
                self.target_shader = None;
                self.transition_progress = 1.0;
            }

            self.last_update = now;
        }
    }

    pub fn current_shader(&self) -> ShaderType {
        self.current_shader
    }

    pub fn is_transitioning(&self) -> bool {
        self.target_shader.is_some()
    }

    pub fn transition_progress(&self) -> f32 {
        self.transition_progress
    }
}

/// Maps audio analysis data to universal uniform structure
pub struct UniformManager {
    start_time: std::time::Instant,
}

impl UniformManager {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }

    pub fn map_audio_data(&self,
                         audio_features: &AudioFeatures,
                         rhythm_features: &RhythmFeatures,
                         resolution: (u32, u32),
                         safety_multipliers: Option<crate::control::safety::SafetyMultipliers>,
                         transition_progress: f32) -> UniversalUniforms {
        let time = self.start_time.elapsed().as_secs_f32();

        UniversalUniforms {
            // 5-band frequency analysis
            sub_bass: audio_features.sub_bass,
            bass: audio_features.bass,
            mid: audio_features.mid,
            treble: audio_features.treble,
            presence: audio_features.presence,

            // Volume and dynamics
            overall_volume: audio_features.overall_volume,
            signal_level_db: audio_features.signal_level_db,
            peak_level_db: audio_features.peak_level_db,
            dynamic_range: audio_features.dynamic_range,

            // Enhanced rhythm analysis
            beat_strength: rhythm_features.beat_strength,
            estimated_bpm: rhythm_features.estimated_bpm,
            tempo_confidence: rhythm_features.tempo_confidence,
            onset_detected: if rhythm_features.onset_detected { 1.0 } else { 0.0 },
            downbeat_detected: if rhythm_features.downbeat_detected { 1.0 } else { 0.0 },

            // Spectral characteristics
            spectral_centroid: audio_features.spectral_centroid,
            spectral_rolloff: audio_features.spectral_rolloff,
            spectral_flux: audio_features.spectral_flux,
            pitch_confidence: audio_features.pitch_confidence,
            zero_crossing_rate: audio_features.zero_crossing_rate,
            onset_strength: audio_features.onset_strength,

            // Time
            time,

            // Resolution
            resolution_x: resolution.0 as f32,
            resolution_y: resolution.1 as f32,

            // Apply safety multipliers if provided
            safety_beat_intensity: safety_multipliers.map(|s| s.beat_intensity).unwrap_or(1.0),
            safety_onset_intensity: safety_multipliers.map(|s| s.onset_intensity).unwrap_or(1.0),
            safety_color_change_rate: safety_multipliers.map(|s| s.color_change_rate).unwrap_or(1.0),
            safety_brightness_range: safety_multipliers.map(|s| s.brightness_range).unwrap_or(1.0),
            safety_pattern_complexity: safety_multipliers.map(|s| s.pattern_complexity).unwrap_or(1.0),
            safety_emergency_stop: safety_multipliers.map(|s| if s.beat_intensity == 0.0 { 0.0 } else { 1.0 }).unwrap_or(1.0),

            // Shader transition blending
            transition_blend: transition_progress,

            // Keep default values for other parameters
            ..UniversalUniforms::default()
        }
    }
}

/// Main shader system that coordinates everything
pub struct ShaderSystem {
    registry: ShaderRegistry,
    transitioner: ShaderTransitioner,
    uniform_manager: UniformManager,
    current_pipeline: Option<wgpu::RenderPipeline>,
    uniform_buffer: Option<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,
    bind_group_layout: wgpu::BindGroupLayout,
    resolution: (u32, u32),
}

impl ShaderSystem {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Result<Self> {
        let registry = ShaderRegistry::new();
        let transitioner = ShaderTransitioner::new(ShaderType::Classic);
        let uniform_manager = UniformManager::new();

        // Create bind group layout for uniforms
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("universal_uniform_bind_group_layout"),
        });

        let mut system = Self {
            registry,
            transitioner,
            uniform_manager,
            current_pipeline: None,
            uniform_buffer: None,
            bind_group: None,
            bind_group_layout,
            resolution: (config.width, config.height),
        };

        // Build initial shader pipeline
        system.rebuild_pipeline(device, config)?;

        Ok(system)
    }

    pub fn set_shader(&mut self, shader_type: ShaderType, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Result<()> {
        if !self.registry.is_available(shader_type) {
            return Err(anyhow!("Shader type {:?} is not available", shader_type));
        }

        self.transitioner.transition_to(shader_type);
        self.rebuild_pipeline(device, config)?;
        Ok(())
    }

    /// Set shader immediately without transition animation (for manual user input)
    pub fn set_shader_immediately(&mut self, shader_type: ShaderType, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Result<()> {
        if !self.registry.is_available(shader_type) {
            return Err(anyhow!("Shader type {:?} is not available", shader_type));
        }

        self.transitioner.switch_immediately_to(shader_type);
        self.rebuild_pipeline(device, config)?;
        Ok(())
    }

    pub fn update(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Result<()> {
        // Update resolution if changed
        let new_resolution = (config.width, config.height);
        if self.resolution != new_resolution {
            self.resolution = new_resolution;
        }

        let was_transitioning = self.transitioner.is_transitioning();
        self.transitioner.update();

        // Rebuild pipeline if transition completed
        if was_transitioning && !self.transitioner.is_transitioning() {
            self.rebuild_pipeline(device, config)?;
        }

        Ok(())
    }

    fn rebuild_pipeline(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Result<()> {
        let current_shader = self.transitioner.current_shader();
        let metadata = self.registry.get(current_shader)
            .ok_or_else(|| anyhow!("Shader metadata not found for {:?}", current_shader))?;

        // Create shader modules
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{}_vertex", metadata.shader_type.name())),
            source: wgpu::ShaderSource::Wgsl(metadata.vertex_source.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{}_fragment", metadata.shader_type.name())),
            source: wgpu::ShaderSource::Wgsl(metadata.fragment_source.into()),
        });

        // Create render pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{}_pipeline_layout", metadata.shader_type.name())),
            bind_group_layouts: &[&self.bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create vertex buffer layout (assuming standard quad)
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress, // pos (3) + tex (2)
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        };

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{}_pipeline", metadata.shader_type.name())),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Create uniform buffer
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("universal_uniform_buffer"),
            contents: bytemuck::cast_slice(&[UniversalUniforms::default()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("universal_uniform_bind_group"),
        });

        self.current_pipeline = Some(pipeline);
        self.uniform_buffer = Some(uniform_buffer);
        self.bind_group = Some(bind_group);

        println!("🎨 Switched to shader: {}", metadata.shader_type.name());

        Ok(())
    }

    pub fn render(&self,
                  device: &wgpu::Device,
                  queue: &wgpu::Queue,
                  view: &wgpu::TextureView,
                  vertex_buffer: &wgpu::Buffer,
                  index_buffer: &wgpu::Buffer,
                  index_count: u32,
                  audio_features: &AudioFeatures,
                  rhythm_features: &RhythmFeatures) -> Result<()> {

        // Update uniforms
        if let Some(ref uniform_buffer) = self.uniform_buffer {
            let transition_progress = self.transitioner.transition_progress();
            let uniforms = self.uniform_manager.map_audio_data(audio_features, rhythm_features, self.resolution, None, transition_progress);
            queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }

        // Render
        if let (Some(ref pipeline), Some(ref bind_group)) = (&self.current_pipeline, &self.bind_group) {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("shader_system_render_encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("shader_system_render_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..index_count, 0, 0..1);
            }

            queue.submit(std::iter::once(encoder.finish()));
        }

        Ok(())
    }

    /// Render with performance quality awareness
    pub fn render_with_quality(&self,
                               device: &wgpu::Device,
                               queue: &wgpu::Queue,
                               view: &wgpu::TextureView,
                               vertex_buffer: &wgpu::Buffer,
                               index_buffer: &wgpu::Buffer,
                               index_count: u32,
                               audio_features: &AudioFeatures,
                               rhythm_features: &RhythmFeatures,
                               quality: QualityLevel,
                               safety_multipliers: Option<crate::control::safety::SafetyMultipliers>) -> Result<()> {

        // Update uniforms with performance parameters
        if let Some(ref uniform_buffer) = self.uniform_buffer {
            let transition_progress = self.transitioner.transition_progress();
            let mut uniforms = self.uniform_manager.map_audio_data(audio_features, rhythm_features, self.resolution, safety_multipliers, transition_progress);

            // Apply quality scaling to audio parameters
            let quality_scale = quality.effect_intensity();
            uniforms.overall_volume *= quality_scale;
            uniforms.color_intensity *= quality_scale;
            uniforms.beat_strength *= quality_scale;

            // Reduce complexity for lower quality levels
            let complexity_scale = quality.complexity_multiplier();
            uniforms.spectral_flux *= complexity_scale;
            uniforms.onset_strength *= complexity_scale;

            queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }

        // Use regular render method for actual rendering
        self.render(device, queue, view, vertex_buffer, index_buffer,
                   index_count, audio_features, rhythm_features)
    }

    pub fn current_shader(&self) -> ShaderType {
        self.transitioner.current_shader()
    }

    pub fn available_shaders(&self) -> Vec<ShaderType> {
        self.registry.available_shaders()
    }

    pub fn is_transitioning(&self) -> bool {
        self.transitioner.is_transitioning()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::{AudioFeatures, RhythmFeatures};
    use crate::control::safety::SafetyMultipliers;

    #[test]
    fn test_uniform_manager_creation() {
        let manager = UniformManager::new();
        // Manager should be created successfully
        assert!(manager.start_time.elapsed().as_secs_f32() >= 0.0);
    }

    #[test]
    fn test_audio_data_mapping_basic() {
        let manager = UniformManager::new();

        let audio_features = AudioFeatures {
            sub_bass: 0.1,
            bass: 0.2,
            mid: 0.3,
            treble: 0.4,
            presence: 0.5,
            overall_volume: 0.6,
            signal_level_db: -20.0,
            peak_level_db: -10.0,
            dynamic_range: 0.7,
            spectral_centroid: 1000.0,
            spectral_rolloff: 2000.0,
            spectral_flux: 0.8,
            pitch_confidence: 0.9,
            zero_crossing_rate: 0.1,
            onset_strength: 0.5,
        };

        let rhythm_features = RhythmFeatures {
            beat_strength: 0.8,
            estimated_bpm: 120.0,
            tempo_bpm: 120.0,
            tempo_confidence: 0.9,
            onset_detected: true,
            downbeat_detected: false,
            rhythm_stability: 0.7,
            beat_position: 0,
        };

        let resolution = (1920, 1080);

        let uniforms = manager.map_audio_data(&audio_features, &rhythm_features, resolution, None, 1.0);

        // Verify audio features are correctly mapped
        assert_eq!(uniforms.sub_bass, 0.1);
        assert_eq!(uniforms.bass, 0.2);
        assert_eq!(uniforms.mid, 0.3);
        assert_eq!(uniforms.treble, 0.4);
        assert_eq!(uniforms.presence, 0.5);
        assert_eq!(uniforms.overall_volume, 0.6);
        assert_eq!(uniforms.signal_level_db, -20.0);
        assert_eq!(uniforms.peak_level_db, -10.0);
        assert_eq!(uniforms.dynamic_range, 0.7);

        // Verify rhythm features are correctly mapped
        assert_eq!(uniforms.beat_strength, 0.8);
        assert_eq!(uniforms.estimated_bpm, 120.0);
        assert_eq!(uniforms.tempo_confidence, 0.9);
        assert_eq!(uniforms.onset_detected, 1.0); // true -> 1.0
        assert_eq!(uniforms.downbeat_detected, 0.0); // false -> 0.0

        // Verify spectral characteristics
        assert_eq!(uniforms.spectral_centroid, 1000.0);
        assert_eq!(uniforms.spectral_rolloff, 2000.0);
        assert_eq!(uniforms.spectral_flux, 0.8);
        assert_eq!(uniforms.pitch_confidence, 0.9);
        assert_eq!(uniforms.zero_crossing_rate, 0.1);
        assert_eq!(uniforms.onset_strength, 0.5);

        // Verify resolution mapping
        assert_eq!(uniforms.resolution_x, 1920.0);
        assert_eq!(uniforms.resolution_y, 1080.0);

        // Verify default safety values (no safety multipliers provided)
        assert_eq!(uniforms.safety_beat_intensity, 1.0);
        assert_eq!(uniforms.safety_onset_intensity, 1.0);
        assert_eq!(uniforms.safety_color_change_rate, 1.0);
        assert_eq!(uniforms.safety_brightness_range, 1.0);
        assert_eq!(uniforms.safety_pattern_complexity, 1.0);
        assert_eq!(uniforms.safety_emergency_stop, 1.0);

        // Verify time is set
        assert!(uniforms.time >= 0.0);
    }

    #[test]
    fn test_safety_multipliers_integration() {
        let manager = UniformManager::new();
        let audio_features = AudioFeatures::new();
        let rhythm_features = RhythmFeatures::new();
        let resolution = (800, 600);

        let safety_multipliers = SafetyMultipliers {
            beat_intensity: 0.3,
            onset_intensity: 0.2,
            color_change_rate: 0.4,
            brightness_range: 0.5,
            pattern_complexity: 0.6,
        };

        let uniforms = manager.map_audio_data(&audio_features, &rhythm_features, resolution, Some(safety_multipliers), 1.0);

        // Verify safety multipliers are correctly applied
        assert_eq!(uniforms.safety_beat_intensity, 0.3);
        assert_eq!(uniforms.safety_onset_intensity, 0.2);
        assert_eq!(uniforms.safety_color_change_rate, 0.4);
        assert_eq!(uniforms.safety_brightness_range, 0.5);
        assert_eq!(uniforms.safety_pattern_complexity, 0.6);
        assert_eq!(uniforms.safety_emergency_stop, 1.0); // beat_intensity > 0, so normal operation
    }

    #[test]
    fn test_emergency_stop_detection() {
        let manager = UniformManager::new();
        let audio_features = AudioFeatures::new();
        let rhythm_features = RhythmFeatures::new();
        let resolution = (800, 600);

        let emergency_safety = SafetyMultipliers {
            beat_intensity: 0.0, // Emergency stop condition
            onset_intensity: 0.0,
            color_change_rate: 0.0,
            brightness_range: 0.1,
            pattern_complexity: 0.0,
        };

        let uniforms = manager.map_audio_data(&audio_features, &rhythm_features, resolution, Some(emergency_safety), 1.0);

        // Verify emergency stop is detected
        assert_eq!(uniforms.safety_emergency_stop, 0.0); // beat_intensity == 0.0 triggers emergency stop
    }

    #[test]
    fn test_boolean_rhythm_conversion() {
        let manager = UniformManager::new();
        let audio_features = AudioFeatures::new();
        let resolution = (1920, 1080);

        // Test true values
        let rhythm_true = RhythmFeatures {
            onset_detected: true,
            downbeat_detected: true,
            ..RhythmFeatures::new()
        };

        let uniforms_true = manager.map_audio_data(&audio_features, &rhythm_true, resolution, None, 1.0);
        assert_eq!(uniforms_true.onset_detected, 1.0);
        assert_eq!(uniforms_true.downbeat_detected, 1.0);

        // Test false values
        let rhythm_false = RhythmFeatures {
            onset_detected: false,
            downbeat_detected: false,
            ..RhythmFeatures::new()
        };

        let uniforms_false = manager.map_audio_data(&audio_features, &rhythm_false, resolution, None, 1.0);
        assert_eq!(uniforms_false.onset_detected, 0.0);
        assert_eq!(uniforms_false.downbeat_detected, 0.0);
    }

    #[test]
    fn test_resolution_conversion() {
        let manager = UniformManager::new();
        let audio_features = AudioFeatures::new();
        let rhythm_features = RhythmFeatures::new();

        // Test different resolutions
        let test_cases = [
            (1920, 1080),
            (1280, 720),
            (800, 600),
            (3840, 2160), // 4K
        ];

        for (width, height) in test_cases {
            let uniforms = manager.map_audio_data(&audio_features, &rhythm_features, (width, height), None, 1.0);
            assert_eq!(uniforms.resolution_x, width as f32);
            assert_eq!(uniforms.resolution_y, height as f32);
        }
    }

    #[test]
    fn test_transition_blend_progress_mapping() {
        let manager = UniformManager::new();
        let audio_features = AudioFeatures::new();
        let rhythm_features = RhythmFeatures::new();
        let resolution = (1920, 1080);

        // Test various transition progress values
        let test_values = vec![0.0, 0.25, 0.5, 0.75, 1.0];

        for progress in test_values {
            let uniforms = manager.map_audio_data(&audio_features, &rhythm_features, resolution, None, progress);
            assert_eq!(uniforms.transition_blend, progress);
        }
    }

    // ===== SHADER SWITCHING VALIDATION TESTS =====

    #[test]
    fn test_shader_registry_completeness() {
        let registry = ShaderRegistry::new();
        let all_shader_types = ShaderType::all();

        // Every shader type should be available in registry
        for &shader_type in all_shader_types {
            assert!(registry.is_available(shader_type), "Shader {:?} should be available", shader_type);
            let metadata = registry.get(shader_type);
            assert!(metadata.is_some(), "Shader {:?} should have metadata", shader_type);
        }

        // Available shaders should match all shader types
        let available = registry.available_shaders();
        assert_eq!(available.len(), all_shader_types.len());
    }

    #[test]
    fn test_shader_transitioner_basic_operations() {
        let mut transitioner = ShaderTransitioner::new(ShaderType::Classic);

        // Initial state
        assert_eq!(transitioner.current_shader(), ShaderType::Classic);
        assert!(!transitioner.is_transitioning());

        // Start transition
        transitioner.transition_to(ShaderType::Plasma);
        assert!(transitioner.is_transitioning());

        // Complete transition (wait for real time to pass)
        std::thread::sleep(std::time::Duration::from_millis(2100));
        transitioner.update();
        assert!(!transitioner.is_transitioning());
        assert_eq!(transitioner.current_shader(), ShaderType::Plasma);
    }

    #[test]
    fn test_shader_transition_progress() {
        let mut transitioner = ShaderTransitioner::new(ShaderType::Classic);
        transitioner.transition_to(ShaderType::Fractal);

        // Transition should start at 0.0 and progress to 1.0
        let initial_progress = transitioner.transition_progress();
        assert_eq!(initial_progress, 0.0);

        // After some time, progress should increase
        std::thread::sleep(std::time::Duration::from_millis(500));
        transitioner.update();
        let mid_progress = transitioner.transition_progress();
        assert!(mid_progress > 0.0 && mid_progress < 1.0);

        // Complete transition
        std::thread::sleep(std::time::Duration::from_millis(1700));
        transitioner.update();
        assert!(!transitioner.is_transitioning());
    }

    #[test]
    fn test_shader_type_properties() {
        // Test all shader types have names and descriptions
        for &shader_type in ShaderType::all() {
            let name = shader_type.name();
            let description = shader_type.description();

            assert!(!name.is_empty(), "Shader {:?} should have a name", shader_type);
            assert!(!description.is_empty(), "Shader {:?} should have a description", shader_type);
        }
    }

    #[test]
    fn test_shader_metadata_validation() {
        let registry = ShaderRegistry::new();

        for &shader_type in ShaderType::all() {
            let metadata = registry.get(shader_type).unwrap();

            // Validate metadata fields
            assert_eq!(metadata.shader_type, shader_type);
            assert!(!metadata.vertex_source.is_empty());
            assert!(!metadata.fragment_source.is_empty());

            // Fragment shader should contain the main function
            assert!(metadata.fragment_source.contains("fs_main"));

            // Should contain UniversalUniforms struct
            assert!(metadata.fragment_source.contains("UniversalUniforms"));
        }
    }

    #[test]
    fn test_shader_switching_sequence() {
        let mut transitioner = ShaderTransitioner::new(ShaderType::Classic);

        let test_sequence = [
            ShaderType::Classic,
            ShaderType::Plasma,
            ShaderType::Tunnel,
            ShaderType::Particle,
            ShaderType::Fractal,
        ];

        for &target_shader in &test_sequence {
            transitioner.transition_to(target_shader);

            // Complete transition
            std::thread::sleep(std::time::Duration::from_millis(2100));
            transitioner.update();

            assert_eq!(transitioner.current_shader(), target_shader);
            assert!(!transitioner.is_transitioning());
        }
    }

    #[test]
    fn test_audio_driven_shader_selection_logic() {
        // Test the shader selection logic from enhanced_composer
        fn analyze_audio_for_shader(audio: &AudioFeatures, rhythm: &RhythmFeatures) -> ShaderType {
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

        // Test bass-driven selections
        let bass_audio = AudioFeatures {
            bass: 0.8,
            sub_bass: 0.3,
            ..AudioFeatures::new()
        };

        let high_tempo_rhythm = RhythmFeatures {
            tempo_confidence: 0.9,
            ..RhythmFeatures::new()
        };
        assert_eq!(analyze_audio_for_shader(&bass_audio, &high_tempo_rhythm), ShaderType::Tunnel);

        let low_tempo_rhythm = RhythmFeatures {
            tempo_confidence: 0.5,
            ..RhythmFeatures::new()
        };
        assert_eq!(analyze_audio_for_shader(&bass_audio, &low_tempo_rhythm), ShaderType::Classic);

        // Test treble + onset -> Particle
        let treble_audio = AudioFeatures {
            treble: 0.7,
            presence: 0.5,
            onset_strength: 0.6,
            ..AudioFeatures::new()
        };
        assert_eq!(analyze_audio_for_shader(&treble_audio, &RhythmFeatures::new()), ShaderType::Particle);

        // Test harmonic content -> Kaleidoscope
        let harmonic_audio = AudioFeatures {
            pitch_confidence: 0.8,
            ..AudioFeatures::new()
        };
        let stable_rhythm = RhythmFeatures {
            rhythm_stability: 0.7,
            ..RhythmFeatures::new()
        };
        assert_eq!(analyze_audio_for_shader(&harmonic_audio, &stable_rhythm), ShaderType::Kaleidoscope);

        // Test spectral flux -> ParametricWave
        let dynamic_audio = AudioFeatures {
            spectral_flux: 0.5,
            ..AudioFeatures::new()
        };
        assert_eq!(analyze_audio_for_shader(&dynamic_audio, &RhythmFeatures::new()), ShaderType::ParametricWave);

        // Test dynamic range -> Fractal
        let range_audio = AudioFeatures {
            dynamic_range: 0.7,
            ..AudioFeatures::new()
        };
        assert_eq!(analyze_audio_for_shader(&range_audio, &RhythmFeatures::new()), ShaderType::Fractal);

        // Test default case
        assert_eq!(analyze_audio_for_shader(&AudioFeatures::new(), &RhythmFeatures::new()), ShaderType::Classic);
    }

    #[test]
    fn test_shader_transition_interruption() {
        let mut transitioner = ShaderTransitioner::new(ShaderType::Classic);

        // Start first transition
        transitioner.transition_to(ShaderType::Plasma);
        assert!(transitioner.is_transitioning());

        // Interrupt with new transition
        transitioner.transition_to(ShaderType::Tunnel);
        assert!(transitioner.is_transitioning());

        // Complete the interrupted transition
        std::thread::sleep(std::time::Duration::from_millis(2100));
        transitioner.update();

        // Should end up at the final target
        assert_eq!(transitioner.current_shader(), ShaderType::Tunnel);
        assert!(!transitioner.is_transitioning());
    }
}