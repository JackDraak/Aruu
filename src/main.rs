use aruu::{AudioProcessor, FeatureMapper};
use std::time::{Duration, Instant};
use std::env;

fn main() -> anyhow::Result<()> {
    println!("🎵 Aruu Audio Visualizer - Phase 1 Demo");

    let mut audio_processor = match AudioProcessor::new() {
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

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let audio_file = &args[1];
        println!("🎶 Loading audio file: {}", audio_file);
        match audio_processor.play_from_file(audio_file) {
            Ok(_) => println!("✅ Successfully loaded audio file"),
            Err(e) => println!("❌ Failed to load audio file: {}", e),
        }
    } else {
        println!("💡 Usage: cargo run [audio_file]");
        println!("   Testing files: sample_gentle.wav, sample_rock.m4a");
    }

    let mut feature_mapper = FeatureMapper::new();

    println!("🚀 Starting audio processing loop...");
    println!("Press Ctrl+C to exit\n");

    let mut frame_count = 0;
    let start_time = Instant::now();
    let target_fps = 60;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    loop {
        let frame_start = Instant::now();

        match audio_processor.process_frame() {
            Ok(features) => {
                let parameters = feature_mapper.map_features_to_parameters(&features);

                if frame_count % 60 == 0 {
                    println!("📊 Audio Features (Frame {}):", frame_count);
                    println!("   🔊 Volume: {:.3}", features.overall_volume);
                    println!("   🔈 Bass: {:.3}, Mid: {:.3}, Treble: {:.3}",
                             features.bass, features.mid, features.treble);
                    println!("   🎼 Spectral Centroid: {:.1} Hz", features.spectral_centroid);
                    println!("   📈 Spectral Rolloff: {:.1} Hz", features.spectral_rolloff);
                    println!();
                    println!("🎨 Shader Parameters:");
                    println!("   💡 Brightness: {:.3}", parameters.overall_brightness);
                    println!("   🌈 Color Intensity: {:.3}", parameters.color_intensity);
                    println!("   📏 Frequency Scale: {:.3}", parameters.frequency_scale);
                    println!("   ⏰ Time Factor: {:.3}", parameters.time_factor);
                    println!("   🎵 Bass/Mid/Treble: {:.3}/{:.3}/{:.3}",
                             parameters.bass_response, parameters.mid_response, parameters.treble_response);
                    println!();
                }
            }
            Err(e) => {
                if frame_count % 300 == 0 {
                    println!("⚠️  Audio processing error: {}", e);
                }
            }
        }

        frame_count += 1;

        let frame_time = frame_start.elapsed();
        if frame_time < frame_duration {
            std::thread::sleep(frame_duration - frame_time);
        }

        if frame_count % 3600 == 0 {
            let elapsed = start_time.elapsed();
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            println!("📈 Performance: {:.1} FPS (Target: {} FPS)", fps, target_fps);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_audio_processing() {
        let mut processor = AudioProcessor::new_default();
        let result = processor.process_frame();
        assert!(result.is_ok());
    }

    #[test]
    fn test_feature_mapping() {
        let mut mapper = FeatureMapper::new();
        let features = aruu::AudioFeatures::new();
        let _parameters = mapper.map_features_to_parameters(&features);
    }
}
