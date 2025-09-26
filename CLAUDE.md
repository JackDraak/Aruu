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

### Completed Phases ✅
1. **Phase 1**: Audio processing foundation
2. **Phase 2**: Enhanced audio-visual pipeline
3. **Phase 3**: Audio-visual synchronization refinements
4. **Phase 4**: Multi-shader architecture integration
5. **Phase 5**: System integration and optimization
6. **Phase 6A**: Core safety infrastructure
7. **Phase 6B**: Shader safety integration (partial - 3/8 shaders)

### Current Phase 🔄
**Phase 6B**: Shader Safety Integration (In Progress)
- ✅ SafetyEngine integrated into rendering pipeline
- ✅ 3 shaders updated with safety parameters (Classic, Parametric, Spectralizer)
- ⏳ 5 remaining shaders need safety implementation (Plasma, Kaleidoscope, Tunnel, Particle, Fractal)
- ✅ Build validation and runtime testing complete

### Upcoming Phases 📋
**Phase 6C**: User Interface Enhancement
- Real-time safety monitoring and status indicators
- Comprehensive safety settings menu
- Performance overlay safety integration
- Break reminder system

**Phase 6D**: Compliance and Testing
- PEAT (Photosensitive Epilepsy Analysis Tool) testing
- WCAG 2.0 compliance verification
- Gaming standards compliance (Xbox, PlayStation, Steam)
- Medical consultation and review

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

## Current Focus Areas

### Immediate (Phase 6B Completion)
- Complete shader safety implementation for remaining 5 shaders
- Apply safety template established by Classic/Parametric/Spectralizer shaders
- Validate build compilation and runtime integration

### Short-term (Phase 6C)
- Enhanced safety monitoring UI
- Real-time safety status indicators
- Break reminder system implementation
- Safety settings menu expansion

### Long-term (Phase 6D)
- Professional compliance testing (PEAT, WCAG 2.0)
- Medical specialist consultation
- Gaming industry standards verification
- Production-ready safety validation

## Developer Information

### Architecture Layers
- **Audio Processing**: Real-time analysis with 15+ parameters
- **Rendering**: 8-shader system with performance scaling
- **Control**: Safety-integrated user interface and controls

### Extension Points
- **New Shaders**: Add to `ShaderType` enum, implement WGSL with `UniversalUniforms`
- **Audio Features**: Extend `AudioFeatures` struct, integrate with `AdvancedAudioAnalyzer`
- **Safety Features**: Modify `SafetyEngine` multipliers, update shader implementations

## Safety Implementation Status

### Core Infrastructure ✅
- SafetyEngine with flash tracking and luminance limiting
- Multi-level safety system (Ultra Safe to Standard)
- Emergency controls and mandatory warning system
- International standards compliance framework

### Shader Integration Status
- ✅ **Spectralizer**: Complete safety implementation
- ✅ **Classic**: Safe bass effects and emergency controls
- ✅ **Parametric Wave**: Safe beat/onset effects and transitions
- ⏳ **Plasma**: Pending safety implementation
- ⏳ **Kaleidoscope**: Pending safety implementation
- ⏳ **Tunnel**: Pending safety implementation
- ⏳ **Particle**: Pending safety implementation
- ⏳ **Fractal**: Pending safety implementation

### Safety Template Established
Each shader receives:
- Safe beat-driven effects with `safety_beat_intensity` multipliers
- Gradual onset transitions with `safety_onset_intensity` controls
- Emergency override with immediate gray fallback
- Brightness limiting with `safety_brightness_range`
- Color clamping and validation

---

**For detailed implementation history, see [HISTORY.md](HISTORY.md)**