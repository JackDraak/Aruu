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
- ✅ Dependencies configured (rodio, wgpu, rustfft, cpal, winit, etc.)
- ✅ Modular architecture implemented
- ✅ Phase 1 Complete: Audio reading + basic FFT
- ✅ All tests passing
- ✅ Build successful (with one deprecation warning)

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

## Dependencies (Implemented)
- rodio: Audio stream processing ✅
- wgpu: GPU rendering ✅
- rustfft: FFT analysis ✅
- cpal: Cross-platform audio I/O ✅
- winit: Window management ✅
- bytemuck: Safe transmutation utilities ✅
- pollster: Async runtime for WGPU ✅
- anyhow: Error handling ✅