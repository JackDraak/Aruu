# Aruu - Audio Visualizer Project

## Project Overview
Real-time audio visualizer using Rust, Rodio for audio processing, and WGPU for rendering with professional-grade epilepsy safety measures.

## Author
JackDraak

## Current Architecture
```
Enhanced Audio Processing Layer
├── Audio Stream Reader (rodio) ✅
├── Advanced FFT Analyzer (5-band frequency analysis) ✅
├── Spectral Feature Extractor (flux, onset, pitch confidence) ✅
├── Dynamic Range Analyzer ✅
└── Enhanced Rhythm Detector (BPM, downbeats) ✅

Multi-Shader Rendering Layer
├── WGPU Context Manager ✅
├── Shader System (hot-swappable 8 shaders) ✅
├── Uniform Manager (40+ parameter unified buffer) ✅
├── Enhanced Frame Composer ✅
└── Performance Scaling System (5 quality levels) ✅

Intelligent Control Layer
├── Enhanced Audio-Visual Mapping ✅
├── Palette Manager (cross-fade transitions) ✅
├── Effect Controller (weight blending) ✅
├── Shader Transitioner ✅
├── User Interface Controller ✅
└── Safety Engine (epilepsy prevention) ✅
```

## Development Status

### Core Functionality ✅
- **Audio Processing Pipeline**: Real-time analysis with 15+ parameters
- **Multi-Shader System**: 8 visual effects with GPU-accelerated rendering
- **Safety Engine**: Epilepsy prevention controls and emergency stop
- **User Controls**: Keyboard interface for shader switching and settings
- **Performance Management**: 5 quality levels for different hardware

### Current Testing Phase 🔄
**Unit Test Validation**
- ✅ Audio→uniform mapping tests (shader_system.rs)
- ✅ Main processing loop tests (visualizer.rs)
- ✅ Safety pipeline tests (SafetyEngine→GPU uniforms)
- ✅ Shader switching validation tests
- ⏳ Full test suite validation pending

### Known Implementation Status
- **Safety Integration**: Partially complete - 3/8 shaders have safety implementations
- **Testing Coverage**: Core pipelines now have comprehensive unit tests
- **Documentation**: Reflects current reality, not future promises

## Current Capabilities ✅

### Audio Analysis (Professional-Grade)
- **5-Band Frequency Analysis**: Sub-Bass, Bass, Mid, Treble, Presence
- **Advanced Spectral Features**: 15+ parameters including flux, onset, pitch confidence
- **Enhanced Rhythm Detection**: BPM estimation, beat strength, downbeat detection
- **Real-time Processing**: <20ms per frame, 60fps capability

### Multi-Shader Rendering System
- **8 Shader Modes**: Classic, ParametricWave, Plasma, Kaleidoscope, Tunnel, Particle, Fractal, Spectralizer
- **Intelligent Auto-Selection**: Audio characteristic-based shader switching
- **Performance Optimization**: 5-level quality system (Ultra to Potato)
- **Universal Parameters**: 40+ parameter unified buffer system

### Safety System (Epilepsy Prevention) 🛡️
- **International Standards Compliance**: WCAG 2.0, ITU, ISO, Gaming Industry
- **Flash Rate Limiting**: ≤3 flashes per second globally
- **Luminance Control**: ≤10% brightness change per update
- **Multi-Level Safety**: Ultra Safe → Standard (5 levels)
- **Emergency Controls**: ESC key instant shutdown
- **Mandatory Warning**: 5-second startup consent screen

## Usage

### Basic Commands
```bash
# Real-time microphone visualization
cargo run

# Audio file visualization
cargo run sample.wav
cargo run sample.m4a

# Run tests
cargo test --lib
```

### Controls
- **1-8**: Direct shader selection
- **Space**: Cycle shaders
- **A**: Toggle auto-shader selection
- **Q**: Cycle quality levels
- **S**: Cycle safety levels
- **ESC**: Emergency stop
- **X**: Resume from emergency stop
- **P**: Toggle performance overlay
- **H**: Help

## Safety Philosophy
**"Maximum Audio Response, Minimum Seizure Risk"**

The goal is to maintain immersive audio-reactive visuals while ensuring safety for all users, including those with photosensitive epilepsy.

## Performance Targets ✅
- Audio processing: <20ms per frame ✅
- Rendering: 60fps (16.67ms/frame) ✅
- Memory usage: <500MB for 30s audio ✅

## Dependencies
- **rodio**: Audio stream processing ✅
- **wgpu**: GPU rendering ✅
- **rustfft**: FFT analysis ✅
- **cpal**: Cross-platform audio I/O ✅
- **winit**: Window management ✅
- **tokio**: Async runtime ✅
- **symphonia**: Extended audio format support ✅
- **anyhow**: Error handling ✅

## System Requirements
- **Minimum**: Any GPU with basic WGPU support
- **Recommended**: Dedicated GPU for Ultra quality
- **Audio**: Any input device or supported audio files
- **OS**: Windows, macOS, Linux

## Current State

### Working Features
- Real-time audio visualization from microphone or files
- 8 different shader effects (Classic, ParametricWave, Plasma, Kaleidoscope, Tunnel, Particle, Fractal, Spectralizer)
- Safety controls including emergency stop (ESC key)
- Performance scaling (5 quality levels)
- Basic keyboard controls for shader switching

### Known Limitations
- Only 3/8 shaders have full safety implementations
- No comprehensive UI for safety settings
- Limited file format support
- Performance not optimized for all hardware

## Developer Information

### Architecture Layers
- **Audio Processing**: Real-time analysis with 15+ parameters
- **Rendering**: 8-shader system with performance scaling
- **Control**: Safety-integrated user interface and controls

### Extension Points
- **New Shaders**: Add to `ShaderType` enum, implement WGSL with `UniversalUniforms`
- **Audio Features**: Extend `AudioFeatures` struct, integrate with `AdvancedAudioAnalyzer`
- **Safety Features**: Modify `SafetyEngine` multipliers, update shader implementations

## Testing Status

### Unit Test Coverage ✅
- **Audio Processing**: Validates feature extraction and rhythm detection
- **Uniform Mapping**: Validates audio→GPU data pipeline
- **Safety System**: Validates epilepsy prevention controls
- **Shader Switching**: Validates transition logic and audio-driven selection
- **Integration**: Validates main processing loops

### Test Results
- Core audio processing pipeline: Functional
- Safety multiplier system: Functional
- Shader registry and transitions: Functional
- Emergency stop controls: Functional

---

**For detailed implementation history, see [HISTORY.md](HISTORY.md)**