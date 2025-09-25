use crate::{AudioProcessor, FeatureMapper, RhythmDetector, RhythmFeatures};
use crate::rendering::{WgpuContext, FrameComposer};
use crate::control::ShaderParameters;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use std::time::Instant;
use anyhow::Result;

pub struct AudioVisualizer {
    audio_processor: AudioProcessor,
    feature_mapper: FeatureMapper,
    rhythm_detector: RhythmDetector,
    wgpu_context: WgpuContext,
    frame_composer: FrameComposer,
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

        let feature_mapper = FeatureMapper::new();
        let rhythm_detector = RhythmDetector::new(44100.0);

        let (wgpu_context, event_loop) = WgpuContext::new().await?;
        let frame_composer = FrameComposer::new(&wgpu_context)?;

        println!("✅ WGPU context and rendering pipeline initialized");
        println!("🚀 Audio Visualizer ready!");

        Ok((
            Self {
                audio_processor,
                feature_mapper,
                rhythm_detector,
                wgpu_context,
                frame_composer,
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
        let audio_features = self.audio_processor.process_frame()?;

        let frequency_bins = vec![
            audio_features.bass,
            audio_features.mid,
            audio_features.treble,
            audio_features.overall_volume,
        ];

        let rhythm_features = self.rhythm_detector.process_frame(&frequency_bins);

        let mut enhanced_parameters = self.feature_mapper.map_features_to_parameters(&audio_features);
        self.apply_rhythm_enhancements(&mut enhanced_parameters, &rhythm_features);

        self.frame_composer.render(&self.wgpu_context, &enhanced_parameters)?;

        Ok(())
    }

    fn apply_rhythm_enhancements(
        &self,
        parameters: &mut ShaderParameters,
        rhythm: &RhythmFeatures,
    ) {
        parameters.time_factor *= 1.0 + rhythm.beat_strength * 0.5;

        if rhythm.onset_detected {
            parameters.overall_brightness *= 1.2;
            parameters.color_intensity *= 1.1;
        }

        let tempo_factor = (rhythm.tempo_bpm - 120.0) / 120.0;
        parameters.frequency_scale *= 1.0 + tempo_factor * 0.2;

        parameters.spectral_shift += rhythm.rhythm_stability * 0.1;
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