# Aruu - Audio Visualizer Project

## Project Overview
Real-time audio visualizer using Rust, Rodio for audio processing, and WGPU for rendering.

## Author
JackDraak (work for hire)

## Architecture
```
Audio Processing Layer
├── Audio Stream Reader (rodio)
├── FFT Analyzer
└── Feature Extractor

Rendering Layer
├── WGPU Context Manager
├── Shader Pipeline
└── Frame Composer

Control Layer
├── Audio-Visual Mapping
└── Dynamic Parameter Controller
```

## Development Phases
1. **Phase 1**: Audio reading + basic FFT (Current)
2. **Phase 2**: Feature extraction + simple shader
3. **Phase 3**: Full parameter mapping + dynamic effects

## Performance Targets
- Audio processing: <20ms per frame
- Rendering: 60fps (16.67ms/frame)
- Memory usage: <500MB for 30s audio

## Current Status
- ✅ Project initialized with Cargo
- ✅ Git repository set up
- ✅ CLAUDE.md working memory created
- ✅ Dependencies configured (rodio, wgpu, rustfft, cpal, winit, tokio, etc.)
- ✅ Modular architecture implemented
- ✅ Phase 1 Complete: Audio reading + basic FFT
- ✅ Phase 2 Complete: Feature extraction + reactive shaders
- ✅ Real-time audio visualization working
- ✅ All tests passing (10 lib + 2 main)
- ✅ Build successful with complete audio-visual pipeline

## Phase 1 Implementation Details

### Audio Processing Module ✅
- `AudioProcessor`: Handles real-time audio input/output with CPAL/Rodio
- `FftAnalyzer`: Performs FFT analysis with Hann windowing (1024 samples)
- `AudioFeatures`: Extracts bass, mid, treble, volume, spectral features

### Control Module ✅
- `ShaderParameters`: 8-parameter struct for visual controls
- `FeatureMapper`: Maps audio features to shader parameters with smoothing

### Rendering Module ✅ (Foundation)
- `WgpuContext`: GPU context management with winit window
- `FrameComposer`: Render pipeline for full-screen quad
- Custom spectral visualization shader (WGSL)

### Main Application ✅
- Real-time 60fps processing loop
- Graceful fallback for audio input failures
- Performance monitoring and telemetry display

## Phase 2 Implementation Details

### Enhanced Audio Processing ✅
- `RhythmDetector`: Beat detection, tempo estimation, onset detection
- `RhythmFeatures`: Beat strength, tempo BPM, rhythm stability
- Enhanced feature mapping with rhythm integration

### Complete Visual Pipeline ✅
- `AudioVisualizer`: Integrated audio-visual application
- Real-time window management with winit events
- 60fps rendering loop with WGPU
- Enhanced reactive shaders with:
  - Radial and angular wave patterns
  - Noise texture for high-frequency detail
  - Dynamic color cycling based on audio
  - Bass-responsive effects and center glow

### Integration Features ✅
- Command-line audio file support
- Real-time microphone input
- Rhythm-enhanced shader parameters
- Onset detection for visual bursts
- Tempo-responsive frequency scaling

## Dependencies (Implemented)
- rodio: Audio stream processing ✅
- wgpu: GPU rendering ✅
- rustfft: FFT analysis ✅
- cpal: Cross-platform audio I/O ✅
- winit: Window management ✅
- bytemuck: Safe transmutation utilities ✅
- pollster: Async runtime for WGPU ✅
- tokio: Async runtime for main application ✅
- anyhow: Error handling ✅

## Usage
```bash
# Real-time microphone visualization
cargo run

# Play audio file with visualization
cargo run sample_gentle.wav
cargo run sample_rock.m4a

# Run tests
cargo test
```

## Next Steps (Phase 3)
- Full parameter mapping for advanced effects
- Multiple visualization modes
- Performance optimizations
- Audio file format expansion