/// Simple multi-shader audio visualizer example
///
/// This example demonstrates the basic usage of the enhanced Aruu system:
/// - Multiple shader modes with automatic selection
/// - User controls for shader switching and quality adjustment
/// - Performance monitoring
///
/// Controls:
/// - 1-8: Select specific shader (Classic, ParametricWave, Plasma, Kaleidoscope, Tunnel, Particle, Fractal, Spectralizer)
/// - Space: Cycle to next shader
/// - A: Toggle auto-shader mode (automatically selects best shader for current audio)
/// - Q: Cycle quality levels (Ultra/High/Medium/Low/Potato)
/// - P: Toggle performance overlay
/// - H: Show help
/// - ESC: Exit

use aruu::AudioVisualizer;
use std::env;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎵 Aruu Multi-Shader Audio Visualizer");
    println!("🚀 Initializing enhanced visualization system...");

    // Create the enhanced audio visualizer
    let (mut visualizer, event_loop) = AudioVisualizer::new().await?;

    // Load audio file if provided
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1];
        println!("🎵 Loading audio file: {}", file_path);
        visualizer.load_audio_file(file_path)?;
    } else {
        println!("🎤 Using microphone input (or silent mode if unavailable)");
    }

    println!("\n🎹 Controls:");
    println!("   1-8: Select shader mode");
    println!("   Space: Next shader");
    println!("   A: Auto-shader toggle");
    println!("   Q: Quality adjustment");
    println!("   P: Performance overlay");
    println!("   H: Help");
    println!("\n🎨 The visualizer will automatically select the best shader for the current audio!");
    println!("💡 Try different music genres to see intelligent shader switching in action.\n");

    // Run the enhanced visualizer
    visualizer.run(event_loop)
}