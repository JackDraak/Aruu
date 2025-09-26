use crate::{AudioProcessor, RhythmDetector};
use crate::rendering::{WgpuContext, EnhancedFrameComposer};
use crate::control::UserInterface;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};
use std::time::Instant;
use anyhow::Result;

pub struct AudioVisualizer {
    audio_processor: AudioProcessor,
    rhythm_detector: RhythmDetector,
    wgpu_context: WgpuContext,
    frame_composer: EnhancedFrameComposer,
    user_interface: UserInterface,
}

impl AudioVisualizer {
    pub async fn new() -> Result<(Self, EventLoop<()>)> {
        println!("🎵 Initializing Aruu Audio Visualizer...");

        let audio_processor = match AudioProcessor::new() {
            Ok(processor) => {
                println!("✅ Audio input initialized successfully");
                processor
            }
            Err(e) => {
                println!("⚠️  Failed to initialize audio input: {}", e);
                println!("💡 Falling back to default processor for testing");
                AudioProcessor::new_default()
            }
        };

        let rhythm_detector = RhythmDetector::new(44100.0);

        let (wgpu_context, event_loop) = WgpuContext::new().await?;
        let frame_composer = EnhancedFrameComposer::new(&wgpu_context)?;
        let user_interface = UserInterface::new();

        println!("✅ WGPU context and rendering pipeline initialized");
        println!("🚀 Audio Visualizer ready!");

        Ok((
            Self {
                audio_processor,
                rhythm_detector,
                wgpu_context,
                frame_composer,
                user_interface,
            },
            event_loop,
        ))
    }

    pub fn run(mut self, event_loop: EventLoop<()>) -> Result<()> {
        let mut last_render_time = Instant::now();
        let target_fps = 60;
        let frame_duration = std::time::Duration::from_millis(1000 / target_fps);

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.wgpu_context.window.id() => match event {
                    WindowEvent::CloseRequested => {
                        println!("👋 Closing Aruu Audio Visualizer");
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        self.wgpu_context.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();
                        if now.duration_since(last_render_time) >= frame_duration {
                            match self.render_frame() {
                                Ok(_) => last_render_time = now,
                                Err(e) => eprintln!("Render error: {}", e),
                            }
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        match self.user_interface.handle_keyboard_input(event, &mut self.frame_composer, &self.wgpu_context) {
                            Ok(handled) => {
                                if handled {
                                    // Display updated status
                                    println!("{}", self.user_interface.get_status_text(&self.frame_composer));
                                }
                            }
                            Err(e) => eprintln!("Keyboard input error: {}", e),
                        }
                    }
                    _ => {}
                }
                Event::AboutToWait => {
                    self.wgpu_context.window.request_redraw();
                }
                _ => {}
            }
        })?;

        Ok(())
    }

    fn render_frame(&mut self) -> Result<()> {
        // Process audio with enhanced features (includes AdvancedAudioAnalyzer internally)
        let audio_features = self.audio_processor.process_frame()?;

        let frequency_bins = vec![
            audio_features.bass,
            audio_features.mid,
            audio_features.treble,
            audio_features.overall_volume,
        ];

        // Enhanced rhythm analysis
        let rhythm_features = self.rhythm_detector.process_frame(&frequency_bins);

        // Auto-select shader based on audio characteristics if enabled
        if self.user_interface.is_auto_shader_enabled() {
            self.frame_composer.auto_select_shader(&self.wgpu_context, &audio_features, &rhythm_features)?;
        }

        // Render with enhanced composer
        self.frame_composer.render(&self.wgpu_context, &audio_features, &rhythm_features)?;

        // Display performance overlay if enabled
        if let Some(performance_text) = self.user_interface.get_performance_overlay(&self.frame_composer) {
            static mut FRAME_COUNTER: u32 = 0;
            unsafe {
                FRAME_COUNTER += 1;
                if FRAME_COUNTER % 60 == 0 { // Display every 60 frames (once per second at 60fps)
                    println!("{}", performance_text);
                }
            }
        }

        Ok(())
    }


    pub fn load_audio_file(&mut self, file_path: &str) -> Result<()> {
        self.audio_processor.play_from_file(file_path)
    }
}

impl Drop for AudioVisualizer {
    fn drop(&mut self) {
        println!("🛑 Audio Visualizer shutting down");
    }
}