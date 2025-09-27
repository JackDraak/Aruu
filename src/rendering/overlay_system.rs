use wgpu::util::DeviceExt;
use anyhow::Result;

use super::{WgpuContext, UniversalUniforms};

/// Types of overlay shaders available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayType {
    DebugOverlay,    // Right-side debug information with transparent white background
    ControlPanel,    // Top-left control panel with UI elements
}

impl OverlayType {
    pub fn name(&self) -> &'static str {
        match self {
            OverlayType::DebugOverlay => "Debug Overlay",
            OverlayType::ControlPanel => "Control Panel",
        }
    }

    pub fn shader_source(&self) -> &'static str {
        match self {
            OverlayType::DebugOverlay => include_str!("shaders/overlay_debug.frag.wgsl"),
            OverlayType::ControlPanel => include_str!("shaders/overlay_control.frag.wgsl"),
        }
    }

    /// Get the screen region this overlay covers (normalized coordinates)
    pub fn screen_region(&self) -> (f32, f32, f32, f32) {
        match self {
            // Debug overlay: right side (x: 0.7-1.0, y: 0.0-1.0)
            OverlayType::DebugOverlay => (0.7, 0.0, 1.0, 1.0),
            // Control panel: top-left (x: 0.0-0.4, y: 0.0-0.3)
            OverlayType::ControlPanel => (0.0, 0.0, 0.4, 0.3),
        }
    }
}

/// Overlay shader metadata and resources
pub struct OverlayShader {
    pub overlay_type: OverlayType,
    pub render_pipeline: wgpu::RenderPipeline,
    pub enabled: bool,
}

/// System for managing and rendering GUI overlay shaders
pub struct OverlaySystem {
    overlays: Vec<OverlayShader>,
    uniform_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: Option<wgpu::BindGroup>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    mouse_position: (f32, f32),
    mouse_pressed: bool,
}

impl OverlaySystem {
    /// Create a new overlay system
    pub fn new(wgpu_context: &WgpuContext) -> Result<Self> {
        let device = &wgpu_context.device;

        // Create uniform buffer for overlay-specific data
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Overlay Uniform Buffer"),
            size: std::mem::size_of::<UniversalUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout for overlays
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Overlay Bind Group Layout"),
            entries: &[
                // Uniform buffer binding
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create vertex and index buffers for overlay quads
        let vertices = create_overlay_vertices();
        let indices = create_overlay_indices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Overlay Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Overlay Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mut overlay_system = Self {
            overlays: Vec::new(),
            uniform_buffer,
            bind_group_layout,
            bind_group: None,
            vertex_buffer,
            index_buffer,
            mouse_position: (0.0, 0.0),
            mouse_pressed: false,
        };

        // Initialize overlay shaders
        overlay_system.initialize_overlays(wgpu_context)?;

        Ok(overlay_system)
    }

    /// Initialize all overlay shaders
    fn initialize_overlays(&mut self, wgpu_context: &WgpuContext) -> Result<()> {
        let device = &wgpu_context.device;

        // Create bind group
        self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Overlay Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
            ],
        }));

        // Common vertex shader for all overlays
        let vertex_shader_source = include_str!("shaders/overlay.vert.wgsl");

        // Create overlay shaders
        for overlay_type in [OverlayType::DebugOverlay, OverlayType::ControlPanel] {
            let overlay_shader = self.create_overlay_shader(
                device,
                &wgpu_context.config,
                overlay_type,
                vertex_shader_source,
            )?;
            self.overlays.push(overlay_shader);
        }

        Ok(())
    }

    /// Create a single overlay shader
    fn create_overlay_shader(
        &self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        overlay_type: OverlayType,
        vertex_source: &str,
    ) -> Result<OverlayShader> {
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{} Vertex Shader", overlay_type.name())),
            source: wgpu::ShaderSource::Wgsl(vertex_source.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{} Fragment Shader", overlay_type.name())),
            source: wgpu::ShaderSource::Wgsl(overlay_type.shader_source().into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{} Pipeline Layout", overlay_type.name())),
            bind_group_layouts: &[&self.bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{} Render Pipeline", overlay_type.name())),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[OverlayVertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // Enable alpha blending for overlays
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None, // No pipeline cache
        });

        Ok(OverlayShader {
            overlay_type,
            render_pipeline,
            enabled: true, // Enable by default
        })
    }

    /// Update overlay system state
    pub fn update(&mut self,
                  mouse_pos: (f32, f32),
                  mouse_pressed: bool,
                  show_debug: bool,
                  show_control: bool) {
        self.mouse_position = mouse_pos;
        self.mouse_pressed = mouse_pressed;

        // Update overlay visibility
        for overlay in &mut self.overlays {
            overlay.enabled = match overlay.overlay_type {
                OverlayType::DebugOverlay => show_debug,
                OverlayType::ControlPanel => show_control,
            };
        }
    }

    /// Render all enabled overlays
    pub fn render(&self,
                  wgpu_context: &WgpuContext,
                  view: &wgpu::TextureView,
                  uniforms: &UniversalUniforms) -> Result<()> {

        // Early return if no overlays are enabled
        let enabled_count = self.overlays.iter().filter(|o| o.enabled).count();
        if enabled_count == 0 {
            return Ok(());
        }

        // Update uniform buffer with current data
        wgpu_context.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[*uniforms]),
        );

        let mut encoder = wgpu_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Overlay Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Overlay Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // Don't clear - we're overlaying
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            if let Some(bind_group) = &self.bind_group {
                render_pass.set_bind_group(0, bind_group, &[]);
            }

            // Render each enabled overlay
            for overlay in &self.overlays {
                if overlay.enabled {
                    render_pass.set_pipeline(&overlay.render_pipeline);
                    render_pass.draw_indexed(0..6, 0, 0..1); // Draw quad (6 indices)
                }
            }
        }

        wgpu_context.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    /// Handle mouse click events and return any UI interactions
    pub fn handle_mouse_click(&self, x: f32, y: f32) -> Vec<OverlayEvent> {
        let mut events = Vec::new();

        for overlay in &self.overlays {
            if !overlay.enabled {
                continue;
            }

            let (min_x, min_y, max_x, max_y) = overlay.overlay_type.screen_region();

            // Check if click is within overlay bounds
            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                // Convert to local coordinates within the overlay
                let local_x = (x - min_x) / (max_x - min_x);
                let local_y = (y - min_y) / (max_y - min_y);

                // Generate events based on overlay type and click position
                events.extend(self.process_overlay_click(overlay.overlay_type, local_x, local_y));
            }
        }

        events
    }

    /// Process clicks within a specific overlay
    fn process_overlay_click(&self, overlay_type: OverlayType, local_x: f32, local_y: f32) -> Vec<OverlayEvent> {
        match overlay_type {
            OverlayType::DebugOverlay => {
                // Debug overlay doesn't have interactive elements currently
                vec![]
            },
            OverlayType::ControlPanel => {
                // ASSUMPTION: Simplified UI layout for control panel
                // Top row: volume control (y: 0.2-0.4)
                // Middle row: file controls (y: 0.4-0.6)
                // Bottom row: safety controls (y: 0.6-0.8)

                if local_y >= 0.2 && local_y <= 0.4 {
                    // Volume control area
                    if local_x >= 0.1 && local_x <= 0.9 {
                        let volume = local_x; // Volume based on X position
                        return vec![OverlayEvent::VolumeChanged(volume)];
                    }
                } else if local_y >= 0.4 && local_y <= 0.6 {
                    // File control area
                    if local_x >= 0.1 && local_x <= 0.3 {
                        return vec![OverlayEvent::OpenFile];
                    } else if local_x >= 0.4 && local_x <= 0.5 {
                        return vec![OverlayEvent::PreviousTrack];
                    } else if local_x >= 0.6 && local_x <= 0.7 {
                        return vec![OverlayEvent::NextTrack];
                    }
                } else if local_y >= 0.6 && local_y <= 0.8 {
                    // Safety control area
                    if local_x >= 0.1 && local_x <= 0.9 {
                        return vec![OverlayEvent::ToggleSafety];
                    }
                }
                vec![]
            }
        }
    }
}

/// Events that can be generated by overlay interactions
#[derive(Debug, Clone)]
pub enum OverlayEvent {
    VolumeChanged(f32),
    OpenFile,
    PreviousTrack,
    NextTrack,
    ToggleSafety,
    EmergencyStop,
}

/// Vertex structure for overlay rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OverlayVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl OverlayVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<OverlayVertex>() as wgpu::BufferAddress,
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
        }
    }
}

/// Create vertices for overlay quads
fn create_overlay_vertices() -> Vec<OverlayVertex> {
    vec![
        // Full-screen quad for overlays
        OverlayVertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
        OverlayVertex { position: [ 1.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
        OverlayVertex { position: [ 1.0,  1.0, 0.0], tex_coords: [1.0, 0.0] },
        OverlayVertex { position: [-1.0,  1.0, 0.0], tex_coords: [0.0, 0.0] },
    ]
}

/// Create indices for overlay quads
fn create_overlay_indices() -> Vec<u16> {
    vec![0, 1, 2, 2, 3, 0]
}