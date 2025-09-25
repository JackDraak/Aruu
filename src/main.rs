use aruu::AudioVisualizer;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🎵 Aruu Audio Visualizer - Phase 2 Demo");

    let (mut visualizer, event_loop) = AudioVisualizer::new().await?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let audio_file = &args[1];
        println!("🎶 Loading audio file: {}", audio_file);
        match visualizer.load_audio_file(audio_file) {
            Ok(_) => println!("✅ Successfully loaded audio file"),
            Err(e) => println!("❌ Failed to load audio file: {}", e),
        }
    } else {
        println!("💡 Usage: cargo run [audio_file]");
        println!("   Testing files: sample_gentle.wav, sample_rock.m4a");
        println!("   Or run without arguments for real-time microphone input");
    }

    println!("🎨 Starting real-time audio visualization...");
    println!("   Close the window or press Ctrl+C to exit");

    visualizer.run(event_loop)
}

#[cfg(test)]
mod tests {
    use aruu::{AudioProcessor, FeatureMapper, AudioFeatures};

    #[test]
    fn test_basic_audio_processing() {
        let mut processor = AudioProcessor::new_default();
        let result = processor.process_frame();
        assert!(result.is_ok());
    }

    #[test]
    fn test_feature_mapping() {
        let mut mapper = FeatureMapper::new();
        let features = AudioFeatures::new();
        let _parameters = mapper.map_features_to_parameters(&features);
    }
}
