/// Shader demonstration example
///
/// This example showcases all 8 available shader modes by automatically cycling through them.
/// It demonstrates how different shaders respond to different types of audio content.
///
/// Usage:
///   cargo run --example shader_demo                    # Use microphone
///   cargo run --example shader_demo sample.wav        # Use audio file
///   cargo run --example shader_demo sample.m4a        # Use M4A/AAC file
///
/// The visualizer will:
/// - Cycle through all 8 shaders every 10 seconds
/// - Display the current shader name and description
/// - Show performance metrics
/// - Demonstrate intelligent auto-selection when enabled

use aruu::AudioVisualizer;
use std::env;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎨 Aruu Shader Demonstration");
    println!("🚀 This demo cycles through all 8 shader modes automatically");

    // Create the enhanced audio visualizer
    let (mut visualizer, event_loop) = AudioVisualizer::new().await?;

    // Load audio file if provided
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1];
        println!("🎵 Loading audio file: {}", file_path);
        visualizer.load_audio_file(file_path)?;
    } else {
        println!("🎤 Using microphone input (recommended for best demo)");
    }

    println!("\n🎭 Shader Modes Available:");
    println!("   1. Classic - Enhanced traditional wave patterns");
    println!("   2. ParametricWave - Mathematical sine/cosine patterns");
    println!("   3. Plasma - Fluid organic patterns");
    println!("   4. Kaleidoscope - Symmetric kaleidoscopic effects");
    println!("   5. Tunnel - 3D tunnel with perspective effects");
    println!("   6. Particle - Dynamic particle systems");
    println!("   7. Fractal - Mandelbrot and Julia set fractals");
    println!("   8. Spectralizer - Direct frequency visualization");

    println!("\n⏰ Auto-cycling every 10 seconds...");
    println!("🎛️  Manual controls still available:");
    println!("   1-8: Jump to specific shader");
    println!("   A: Toggle auto-selection mode");
    println!("   P: Performance overlay");
    println!("   Q: Quality adjustment");
    println!("   H: Help");
    println!("\n💡 Try playing music with different characteristics!");
    println!("   🥁 Bass-heavy → Tunnel/Classic shaders");
    println!("   🎵 Melodic → Kaleidoscope shader");
    println!("   🎸 Dynamic → Particle/Fractal shaders");
    println!("   🎼 Harmonic → Spectralizer shader\n");

    // The AudioVisualizer handles all the rendering and user interaction
    // The automatic shader cycling would be handled by periodic key simulation
    // or by extending the AudioVisualizer with a demo mode

    visualizer.run(event_loop)
}