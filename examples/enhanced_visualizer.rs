/// Enhanced audio visualizer example demonstrating the complete multi-shader system
/// with user controls, performance optimization, and intelligent shader selection
///
/// This example showcases:
/// - 8 different shader modes with audio-reactive parameters
/// - Performance optimization system with quality scaling
/// - Real-time user controls for shader selection and quality adjustment
/// - Intelligent auto-shader selection based on audio characteristics
///
/// Controls:
/// - 1-8: Direct shader selection
/// - Space: Next shader
/// - Tab: Previous shader
/// - A: Toggle auto shader mode
/// - Q/W/E/R/T: Quality levels (Potato/Low/Medium/High/Ultra)
/// - Y: Auto quality
/// - P: Toggle performance overlay
/// - H/F1: Help

use aruu::*;
use aruu::control::UserInterface;
use aruu::rendering::{EnhancedFrameComposer, WgpuContext};

use anyhow::Result;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::window::{Window, WindowId};
use tokio::sync::mpsc;

struct EnhancedVisualizerApp {
    // Core components
    window: Option<Window>,
    context: Option<WgpuContext>,
    composer: Option<EnhancedFrameComposer>,

    // Audio processing
    audio_processor: Option<AudioProcessor>,
    feature_mapper: FeatureMapper,
    rhythm_detector: RhythmDetector,

    // User interface
    user_interface: UserInterface,

    // Audio data channels
    audio_receiver: Option<mpsc::UnboundedReceiver<Vec<f32>>>,

    // Timing
    last_frame: Instant,
    frame_count: u32,
}

impl EnhancedVisualizerApp {
    fn new() -> Result<Self> {
        Ok(Self {
            window: None,
            context: None,
            composer: None,
            audio_processor: None,
            feature_mapper: FeatureMapper::new(),
            rhythm_detector: RhythmDetector::new(44100.0), // Standard sample rate
            user_interface: UserInterface::new(),
            audio_receiver: None,
            last_frame: Instant::now(),
            frame_count: 0,
        })
    }

    fn setup_audio(&mut self) -> Result<()> {
        let (audio_tx, audio_rx) = mpsc::unbounded_channel();
        self.audio_receiver = Some(audio_rx);

        // Initialize audio processor in background thread
        let mut processor = AudioProcessor::new(44100.0)?;

        tokio::spawn(async move {
            loop {
                if let Ok(samples) = processor.read_samples(1024).await {
                    if !samples.is_empty() {
                        let _ = audio_tx.send(samples);
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });

        println!("🎵 Enhanced Aruu Audio Visualizer");
        println!("📖 Press H or F1 for help with controls");
        Ok(())
    }

    fn process_audio_frame(&mut self) -> (AudioFeatures, RhythmFeatures) {
        // Try to get latest audio data
        let mut latest_samples = None;
        if let Some(ref mut rx) = self.audio_receiver {
            while let Ok(samples) = rx.try_recv() {
                latest_samples = Some(samples);
            }
        }

        // Process audio or use silence
        let samples = latest_samples.unwrap_or_else(|| vec![0.0; 1024]);

        // Extract features
        let audio_features = self.feature_mapper.extract_features(&samples);
        let rhythm_features = self.rhythm_detector.analyze(&samples);

        // Map and smooth features
        self.feature_mapper.update_features(&audio_features, &rhythm_features);

        (audio_features, rhythm_features)
    }

    fn update_auto_shader(&mut self) -> Result<()> {
        if let (Some(ref mut composer), Some(ref context)) = (&mut self.composer, &self.context) {
            if self.user_interface.is_auto_shader_enabled() {
                let (audio_features, rhythm_features) = self.process_audio_frame();
                composer.auto_select_shader(context, &audio_features, &rhythm_features)?;
            }
        }
        Ok(())
    }

    fn render_frame(&mut self) -> Result<()> {
        if let (Some(ref mut composer), Some(ref context)) = (&mut self.composer, &self.context) {
            let (audio_features, rhythm_features) = self.process_audio_frame();

            // Render with enhanced composer (includes performance monitoring)
            composer.render(context, &audio_features, &rhythm_features)?;

            // Update frame statistics
            self.frame_count += 1;
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_frame);

            // Print status every second
            if elapsed.as_secs() >= 1 {
                let status = self.user_interface.get_status_text(composer);
                println!("🎨 {}", status);

                // Print performance overlay if enabled
                if let Some(overlay) = self.user_interface.get_performance_overlay(composer) {
                    println!("{}", overlay);
                }

                self.last_frame = now;
                self.frame_count = 0;
            }
        }
        Ok(())
    }
}

impl ApplicationHandler for EnhancedVisualizerApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = winit::window::WindowAttributes::default()
                .with_title("Enhanced Aruu Audio Visualizer - Multi-Shader System")
                .with_inner_size(winit::dpi::PhysicalSize::new(1200, 800));

            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    // Initialize graphics context
                    match pollster::block_on(WgpuContext::new(&window)) {
                        Ok(context) => {
                            // Initialize enhanced frame composer
                            match EnhancedFrameComposer::new(&context) {
                                Ok(composer) => {
                                    self.context = Some(context);
                                    self.composer = Some(composer);
                                    self.window = Some(window);

                                    // Setup audio processing
                                    if let Err(e) = self.setup_audio() {
                                        eprintln!("⚠️  Audio setup failed: {}", e);
                                        eprintln!("🔇 Continuing with silent mode");
                                    }

                                    println!("✅ Enhanced visualizer initialized successfully");
                                    println!("🎮 {} shaders available with intelligent selection",
                                            self.composer.as_ref().unwrap().available_shaders().len());
                                }
                                Err(e) => eprintln!("❌ Failed to create enhanced composer: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to create WGPU context: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Failed to create window: {}", e),
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("👋 Enhanced Aruu shutting down");
                event_loop.exit();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let (Some(ref mut composer), Some(ref context)) = (&mut self.composer, &self.context) {
                    match self.user_interface.handle_keyboard_input(&event, composer, context) {
                        Ok(handled) => {
                            if handled {
                                // Key was handled by UI system
                            }
                        }
                        Err(e) => eprintln!("⚠️  Input handling error: {}", e),
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                if let Err(e) = self.update_auto_shader() {
                    eprintln!("⚠️  Auto shader update failed: {}", e);
                }

                if let Err(e) = self.render_frame() {
                    eprintln!("⚠️  Render error: {}", e);
                }
            }

            WindowEvent::Resized(new_size) => {
                if let Some(ref mut context) = self.context {
                    context.resize(new_size);
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(ref window) = self.window {
            window.request_redraw();
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Starting Enhanced Aruu Audio Visualizer");

    let event_loop = EventLoop::new()?;
    let mut app = EnhancedVisualizerApp::new()?;

    println!("🎛️  Features enabled:");
    println!("   🎨 8 shader modes with intelligent selection");
    println!("   📊 Real-time performance optimization");
    println!("   🎮 Interactive controls for quality and shader selection");
    println!("   🔄 Automatic shader transitions based on music");
    println!("   📈 Performance monitoring and adaptive quality scaling");

    event_loop.run_app(&mut app)?;
    Ok(())
}