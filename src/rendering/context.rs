use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{
    event_loop::EventLoop,
    window::Window,
};
use anyhow::Result;
use std::sync::Arc;

pub struct WgpuContext {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,
}

impl WgpuContext {
    pub async fn new() -> Result<(Self, EventLoop<()>)> {
        let event_loop = EventLoop::new()?;
        let window = Arc::new(event_loop
            .create_window(winit::window::WindowAttributes::default()
                .with_title("Aruu Audio Visualizer")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600)))?);

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find an appropriate adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        // Select present mode with preference for V-sync (60 FPS cap)
        let present_mode = surface_caps
            .present_modes
            .iter()
            .copied()
            .find(|&mode| mode == wgpu::PresentMode::Fifo)
            .or_else(|| {
                // Fallback hierarchy for V-sync-like behavior
                surface_caps
                    .present_modes
                    .iter()
                    .copied()
                    .find(|&mode| mode == wgpu::PresentMode::FifoRelaxed)
            })
            .unwrap_or(surface_caps.present_modes[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Log the selected present mode for verification
        let mode_name = match present_mode {
            wgpu::PresentMode::Fifo => "Fifo (V-sync, 60 FPS)",
            wgpu::PresentMode::FifoRelaxed => "FifoRelaxed (Adaptive V-sync)",
            wgpu::PresentMode::Immediate => "Immediate (Unlimited FPS)",
            wgpu::PresentMode::Mailbox => "Mailbox (Triple buffering)",
            wgpu::PresentMode::AutoVsync => "AutoVsync",
            wgpu::PresentMode::AutoNoVsync => "AutoNoVsync",
        };
        println!("🖥️  Present mode: {}", mode_name);

        let context = Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        };

        Ok((context, event_loop))
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture> {
        self.surface
            .get_current_texture()
            .map_err(|e| anyhow::anyhow!("Failed to acquire next swap chain texture: {}", e))
    }
}