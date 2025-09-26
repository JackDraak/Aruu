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

        event_loop.run(move |event, elwt| { // ASSUMPTION: Keeping deprecated API for simplicity - requires major refactoring to fix
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

                                // Check for exit condition (double ESC press)
                                if self.user_interface.should_exit() {
                                    println!("👋 Closing Aruu Audio Visualizer");
                                    elwt.exit();
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

        // Render with enhanced composer and safety multipliers
        let safety_multipliers = self.user_interface.get_safety_multipliers();
        self.frame_composer.render(&self.wgpu_context, &audio_features, &rhythm_features, Some(safety_multipliers))?;

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

#[cfg(test)]
mod tests {
    use super::*;

    // Integration test for the core audio processing pipeline
    #[test]
    fn test_audio_processing_pipeline() {
        // Use default processor for testing (no audio device required)
        let mut audio_processor = AudioProcessor::new_default();
        let mut rhythm_detector = RhythmDetector::new(44100.0);

        // Process a frame to ensure pipeline works
        let audio_features = audio_processor.process_frame().expect("Audio processing should work");

        // Verify audio features are generated
        assert!(audio_features.sub_bass >= 0.0);
        assert!(audio_features.bass >= 0.0);
        assert!(audio_features.mid >= 0.0);
        assert!(audio_features.treble >= 0.0);
        assert!(audio_features.presence >= 0.0);
        assert!(audio_features.overall_volume >= 0.0);

        // Test rhythm processing with audio features
        let frequency_bins = vec![
            audio_features.bass,
            audio_features.mid,
            audio_features.treble,
            audio_features.overall_volume,
        ];

        let rhythm_features = rhythm_detector.process_frame(&frequency_bins);

        // Verify rhythm features are generated
        assert!(rhythm_features.beat_strength >= 0.0);
        assert!(rhythm_features.estimated_bpm >= 0.0);
        assert!(rhythm_features.tempo_confidence >= 0.0);
        assert!(rhythm_features.rhythm_stability >= 0.0);
        // beat_position is u8 so >= 0 check is redundant
    }

    #[test]
    fn test_multiple_frame_processing() {
        let mut audio_processor = AudioProcessor::new_default();
        let mut rhythm_detector = RhythmDetector::new(44100.0);

        // Process multiple frames to ensure stability
        for _ in 0..10 {
            let audio_features = audio_processor.process_frame().expect("Audio processing should work");

            let frequency_bins = vec![
                audio_features.bass,
                audio_features.mid,
                audio_features.treble,
                audio_features.overall_volume,
            ];

            let _rhythm_features = rhythm_detector.process_frame(&frequency_bins);
            // Should not panic or fail
        }
    }

    #[test]
    fn test_user_interface_integration() {
        let user_interface = UserInterface::new();

        // Test basic UI functions work
        assert!(user_interface.is_auto_shader_enabled()); // Default should be true
        assert!(!user_interface.is_emergency_stopped()); // Should not start in emergency stop

        // Test safety multipliers are accessible
        let safety_multipliers = user_interface.get_safety_multipliers();

        // Default should allow normal operation
        assert!(safety_multipliers.beat_intensity > 0.0);
        assert!(safety_multipliers.onset_intensity > 0.0);
        assert!(safety_multipliers.brightness_range > 0.0);
    }

    #[test]
    fn test_audio_file_loading() {
        let mut audio_processor = AudioProcessor::new_default();

        // Test that loading a non-existent file fails gracefully
        let result = audio_processor.play_from_file("nonexistent_file.wav");

        // Should return an error but not panic
        assert!(result.is_err());
    }

    #[test]
    fn test_audio_features_validity() {
        let mut audio_processor = AudioProcessor::new_default();

        let audio_features = audio_processor.process_frame().expect("Should process frame");

        // Verify all audio features are valid numbers (not NaN or infinite)
        assert!(audio_features.sub_bass.is_finite());
        assert!(audio_features.bass.is_finite());
        assert!(audio_features.mid.is_finite());
        assert!(audio_features.treble.is_finite());
        assert!(audio_features.presence.is_finite());
        assert!(audio_features.overall_volume.is_finite());
        assert!(audio_features.dynamic_range.is_finite());
        assert!(audio_features.spectral_centroid.is_finite());
        assert!(audio_features.spectral_rolloff.is_finite());
        assert!(audio_features.spectral_flux.is_finite());
        assert!(audio_features.pitch_confidence.is_finite());
        assert!(audio_features.zero_crossing_rate.is_finite());
        assert!(audio_features.onset_strength.is_finite());

        // Verify features are in expected ranges
        assert!(audio_features.sub_bass >= 0.0 && audio_features.sub_bass <= 1.0);
        assert!(audio_features.bass >= 0.0 && audio_features.bass <= 1.0);
        assert!(audio_features.mid >= 0.0 && audio_features.mid <= 1.0);
        assert!(audio_features.treble >= 0.0 && audio_features.treble <= 1.0);
        assert!(audio_features.presence >= 0.0 && audio_features.presence <= 1.0);
        assert!(audio_features.overall_volume >= 0.0 && audio_features.overall_volume <= 1.0);
    }

    #[test]
    fn test_rhythm_features_validity() {
        let mut rhythm_detector = RhythmDetector::new(44100.0);

        // Test with various frequency patterns
        let test_bins = vec![0.5, 0.3, 0.2, 0.8];
        let rhythm_features = rhythm_detector.process_frame(&test_bins);

        // Verify all rhythm features are valid numbers
        assert!(rhythm_features.beat_strength.is_finite());
        assert!(rhythm_features.estimated_bpm.is_finite());
        assert!(rhythm_features.tempo_confidence.is_finite());
        assert!(rhythm_features.rhythm_stability.is_finite());
        // beat_position is u8 so <= 255 check is redundant

        // Verify features are in expected ranges
        assert!(rhythm_features.beat_strength >= 0.0 && rhythm_features.beat_strength <= 1.0);
        assert!(rhythm_features.tempo_confidence >= 0.0 && rhythm_features.tempo_confidence <= 1.0);
        assert!(rhythm_features.rhythm_stability >= 0.0 && rhythm_features.rhythm_stability <= 1.0);
        // beat_position is u8 so <= 255 check is redundant
        assert!(rhythm_features.estimated_bpm >= 60.0 && rhythm_features.estimated_bpm <= 200.0); // Reasonable BPM range
    }
}