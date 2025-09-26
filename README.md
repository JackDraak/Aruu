# 🎵 Aruu - Professional Audio Visualizer

A real-time, multi-shader audio visualizer built in Rust with WGPU, featuring intelligent shader selection and comprehensive epilepsy safety measures.

## ✨ Features

### 🎨 **8 Intelligent Shader Modes**
- **Classic**: Enhanced traditional wave patterns
- **ParametricWave**: Mathematical sine/cosine patterns
- **Plasma**: Fluid organic patterns driven by low frequencies
- **Kaleidoscope**: Symmetric patterns for harmonic content
- **Tunnel**: 3D perspective effects with bass-driven depth
- **Particle**: Dynamic particle systems for transients
- **Fractal**: Mandelbrot/Julia sets scaled by spectral characteristics
- **Spectralizer**: Direct frequency visualization with artistic flair

### 🤖 **Intelligent Auto-Selection**
Automatically selects optimal shaders based on real-time audio analysis:
- **Bass-Heavy** → Classic/Tunnel shaders
- **Treble + Transients** → Particle shader
- **Harmonic Content** → Kaleidoscope shader
- **Dynamic Changes** → ParametricWave shader
- **High Dynamic Range** → Fractal shader

### 🛡️ **Epilepsy Safety System** ⚠️
- **WCAG 2.0 Compliant**: ≤3 flashes/second, ≤10% luminance change
- **International Standards**: Meets Xbox, PlayStation, Steam safety requirements
- **Smart Safety Engine**: Flash rate limiting and luminance control
- **Multiple Safety Levels**: Ultra Safe → Safe → Moderate → Standard
- **Emergency Stop**: Instant visual shutdown (ESC key)
- **Startup Warning**: Mandatory epilepsy awareness screen

### 🎵 **Professional Audio Analysis**
- **5-Band Frequency Analysis**: Sub-Bass, Bass, Mid, Treble, Presence
- **Advanced Features**: Spectral flux, onset detection, pitch confidence
- **Rhythm Detection**: BPM estimation with confidence metrics
- **Dynamic Range**: Volume variation analysis
- **15+ Audio Parameters**: Comprehensive real-time analysis

### ⚡ **Performance Optimization**
- **V-sync Frame Limiting**: Proper 60 FPS instead of unlimited
- **Quality Scaling**: Ultra → High → Medium → Low → Potato → Auto
- **Adaptive Performance**: Automatically adjusts based on hardware
- **Multi-threaded**: Separate audio and rendering threads

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/JackDraak/Aruu.git
cd Aruu

# Run with microphone input
cargo run

# Run with audio file
cargo run sample.wav

# Run shader demonstration
cargo run --example shader_demo sample.wav

# Run simple multi-shader example
cargo run --example simple_multi_shader
```

## 🎮 Controls

### **Shader Selection**
- `1-8` - Direct shader selection
- `Space` - Cycle to next shader
- `A` - Toggle intelligent auto-shader mode ⭐

### **Safety & Quality**
- `ESC` - Emergency visual stop 🛡️
- `S` - Toggle Safety Mode
- `Q` - Cycle quality levels
- `P` - Performance overlay
- `H` - Help and status

### **Safety Levels**
- 🛡️ **Ultra Safe**: Maximum epilepsy protection
- 🔒 **Safe**: Conservative for general use (default)
- ⚠️ **Moderate**: Balanced safety and visuals
- 🎨 **Standard**: Full features for experienced users

## ⚠️ Safety Warning

**PHOTOSENSITIVE EPILEPSY WARNING**: This software contains flashing lights and visual effects. Do not use if you have a history of seizures or epilepsy. The application includes mandatory safety warnings and multiple protection levels.

**Safety Features**:
- Startup warning screen with user consent
- Multiple safety levels with intelligent limiting
- Emergency stop controls (ESC key)
- WCAG 2.0 compliance for accessibility
- Real-time safety monitoring

## 🎼 Supported Audio Formats

- **Real-time**: Microphone input (CPAL)
- **WAV**: PCM audio files
- **M4A/AAC**: Advanced Audio Coding files

## 🛠️ System Requirements

### **Minimum**
- Any GPU with basic WGPU support
- Audio input device or supported audio files
- Windows, macOS, or Linux

### **Recommended**
- Dedicated GPU for Ultra quality mode
- High-quality audio interface for best analysis
- 60Hz+ display for smooth V-sync operation

## 🧑‍💻 Development

### **Architecture**
- **Audio Layer**: CPAL, Rodio, RustFFT for real-time processing
- **Rendering Layer**: WGPU with 8 specialized WGSL shaders
- **Control Layer**: Intelligent audio-visual mapping with safety systems
- **Safety Layer**: Comprehensive epilepsy prevention engine

### **Key Dependencies**
- `wgpu` - GPU rendering and compute
- `winit` - Cross-platform windowing
- `rodio` - Audio streaming and decoding
- `rustfft` - Real-time frequency analysis
- `cpal` - Cross-platform audio I/O
- `tokio` - Async runtime

### **Building from Source**
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test --lib

# Check WGSL shaders
cargo check
```

## 📊 Performance Tips

### **High-End Systems**
- Use **Ultra** quality for maximum visual impact
- Enable all shader effects for best experience
- Monitor GPU usage with performance overlay

### **Mid-Range Systems**
- **High** or **Medium** quality provides excellent balance
- Auto-shader mode adapts to system performance
- V-sync prevents unnecessary GPU load

### **Lower-End Systems**
- Enable **Auto** quality for adaptive performance
- Use **Potato** mode for minimal resource usage
- **Safety Mode** also reduces computational load

## 🎯 Use Cases

### **Music Genres**
- **Electronic/EDM** → Tunnel, Particle shaders
- **Classical/Orchestral** → Kaleidoscope, Fractal shaders
- **Rock/Metal** → Particle shader for transients
- **Jazz** → Fractal shader for dynamic range
- **Ambient** → Plasma shader for organic flow

### **Applications**
- Live music visualization
- Audio production monitoring
- Creative installations
- Educational demonstrations
- Accessibility-conscious entertainment

## 🤝 Contributing

Contributions welcome! Please ensure all submissions:
- Include comprehensive epilepsy safety testing
- Follow existing code style and documentation
- Include unit tests for new features
- Maintain WCAG 2.0 accessibility compliance

## 📜 License

This project is licensed under the MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- International epilepsy prevention standards (WCAG 2.0, ITU, ISO)
- Gaming industry safety guidelines (Xbox, PlayStation, Steam)
- Rust audio and graphics communities
- Medical research on photosensitive epilepsy

---

**Safety Philosophy**: *"Maximum Audio Response, Minimum Seizure Risk"*

**Created by**: JackDraak
**Status**: Active Development with Epilepsy Safety Priority

⚠️ **Remember**: Always prioritize user safety over visual effects. When in doubt, err on the side of caution.