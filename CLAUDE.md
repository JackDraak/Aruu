# Aruu - Audio Visualizer Project

## Project Overview
Real-time audio visualizer using Rust, Rodio for audio processing, and WGPU for rendering.

## Author
JackDraak (work for hire)

## Architecture
```
Enhanced Audio Processing Layer
├── Audio Stream Reader (rodio)
├── Advanced FFT Analyzer (5-band frequency analysis)
├── Spectral Feature Extractor (flux, onset, pitch confidence)
├── Dynamic Range Analyzer
└── Enhanced Rhythm Detector (BPM, downbeats)

Multi-Shader Rendering Layer
├── WGPU Context Manager
├── Shader System (hot-swappable shaders)
├── Uniform Manager (unified buffer management)
├── Frame Composer
└── Performance Scaling System

Intelligent Control Layer
├── Enhanced Audio-Visual Mapping
├── Palette Manager (cross-fade transitions)
├── Effect Controller (weight blending)
├── Shader Transitioner
└── User Interface Controller
```

## Development Phases
1. **Phase 1**: Audio reading + basic FFT ✅
2. **Phase 2**: Feature extraction + reactive shader ✅
3. **Phase 3**: Audio-visual synchronization refinements ✅
4. **Phase 4**: Multi-shader architecture (In Progress)
5. **Phase 5**: Advanced visual effects and optimization

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
- Command-line audio file support (WAV, M4A/AAC)
- Real-time microphone input
- Rhythm-enhanced shader parameters
- Onset detection for visual bursts
- Tempo-responsive frequency scaling

## Phase 3 Audio-Visual Synchronization Refinements ✅

### Downbeat-Only Palette Switching ✅
- Enhanced rhythm detector with 4/4 time signature tracking
- Downbeat detection requiring high beat strength (>0.7) on beat position 0
- Increased palette switch cooldown to 2.0 seconds for musical pacing
- Professional-grade palette cycling aligned with musical structure

### Cross-Fade Shader Transition System ✅
- Palette transition parameters: transition_blend, prev_palette_*
- 1-second smooth cross-fade using smoothstep curve
- Enhanced WGSL shader to blend between previous and current palettes
- Eliminated jarring palette jumps with cinematic transitions

### Enhanced Volume-Based Saturation ✅
- Complete desaturation below -50dB (true silence)
- Gradual ramp with max 30% saturation until -30dB
- Exponential curve from -30dB to -6dB for dramatic effect
- Clear visual correlation between audio levels and color saturation

### Color Palette System ✅
- 8 distinct palettes: Rainbow, Red, Orange, Yellow, Green, Blue, Indigo, Violet
- Intelligent hue-based color generation
- Smooth palette transitions with PaletteManager
- Beat-synchronized palette switching on downbeats

## Enhanced Smoothing System ✅

### Advanced State Transition Smoothing
- `Smoother`: Generalized smoothing system for visual transitions
- **Linear Smoothing**: Constant rate transitions
- **Exponential Smoothing**: Natural decay-based transitions
- **Adaptive Smoothing**: Dynamic response based on change rate

### Per-Parameter Smoothing Configuration
- **Bass Response**: Adaptive (0.1-0.6 factor, high sensitivity)
- **Mid Response**: Adaptive (0.08-0.4 factor, moderate sensitivity)
- **Treble Response**: Adaptive (0.05-0.5 factor, very responsive)
- **Overall Brightness**: Exponential decay (3.0)
- **Color Intensity**: Adaptive (0.05-0.3 factor, high sensitivity)
- **Frequency Scale**: Exponential decay (2.0)
- **Spectral Shift**: Exponential decay (1.5)

### Benefits
- Eliminates visual jitter and harsh transitions
- Maintains responsiveness for musical elements
- Different smoothing strategies optimized per parameter type
- Adaptive smoothing responds faster to significant changes

## Dependencies (Implemented)
- rodio: Audio stream processing with Symphonia features ✅
- wgpu: GPU rendering ✅
- rustfft: FFT analysis ✅
- cpal: Cross-platform audio I/O ✅
- winit: Window management ✅
- bytemuck: Safe transmutation utilities ✅
- pollster: Async runtime for WGPU ✅
- tokio: Async runtime for main application ✅
- symphonia: Extended audio format support (AAC, M4A) ✅
- anyhow: Error handling ✅

## Usage
```bash
# Real-time microphone visualization
cargo run

# Play audio file with visualization (supports WAV, M4A/AAC)
cargo run sample_gentle.wav
cargo run sample_rock.m4a

# Run tests (16 total: 14 lib + 2 main)
cargo test
```

## Audio Format Support
- **WAV**: PCM audio files
- **M4A/AAC**: Advanced Audio Coding files
- **Real-time**: Microphone input with CPAL

## Smoothing System Usage
```rust
// Configure custom smoothing for specific parameters
let mut mapper = FeatureMapper::new();
mapper.configure_smoothing("bass_response", SmoothingType::linear(0.8));
mapper.configure_smoothing("brightness", SmoothingType::exponential(5.0));
mapper.configure_smoothing("color", SmoothingType::adaptive(0.1, 0.7, 3.0));
```

## Phase 4: Multi-Shader Architecture Integration (In Progress)

### Enhanced Audio Analysis Pipeline 🔄
**Objective**: Expand to professional-grade audio analysis with 15+ parameters

#### 5-Band Frequency Analysis
- **Sub-Bass**: 20-60 Hz (deep low-end content)
- **Bass**: 60-200 Hz (fundamental bass frequencies)
- **Mid**: 200-2000 Hz (vocal and instrument fundamentals)
- **Treble**: 2000-8000 Hz (clarity and presence)
- **Presence**: 8000+ Hz (air and sparkle)

#### Advanced Spectral Features
- **Spectral Flux**: Frame-to-frame spectral difference for texture variation
- **Onset Strength**: Transient detection for sudden visual changes
- **Pitch Confidence**: Harmonic content analysis for color harmonies
- **Dynamic Range**: RMS variation over time windows
- **Enhanced BPM**: More accurate tempo estimation with confidence metrics

### Multi-Shader Architecture 🔄
**Objective**: Flexible shader system supporting multiple visual modes

#### Core Shader System Components
- **ShaderRegistry**: Manages available shaders and metadata
- **ShaderTransitioner**: Handles smooth transitions between visual modes
- **UniformManager**: Maps audio data to different shader uniform layouts
- **EffectController**: Manages effect weights and real-time blending

#### Available Shader Modes
1. **Classic Mode**: Current implementation (backward compatibility)
2. **Parametric Wave**: Mathematical patterns with audio-reactive parameters
3. **Plasma Mode**: Fluid, organic patterns driven by low frequencies
4. **Kaleidoscope Mode**: Symmetric patterns responding to harmonic content
5. **Tunnel Mode**: 3D perspective effects with bass-driven depth
6. **Particle Mode**: Dynamic particle systems responding to transients
7. **Fractal Mode**: Self-similar patterns scaled by spectral characteristics
8. **Spectralizer Mode**: Direct frequency visualization with artistic flair

### Intelligent Color Systems 🔄
**Objective**: Replace simple palettes with frequency-aware color generation

#### Frequency Dominance Analysis
- **Bass-Heavy Music** → Warm colors (reds, oranges)
- **Treble-Heavy Music** → Cool colors (blues, cyans)
- **Mid-Heavy Music** → Green/yellow spectrum
- **Harmonic Content** → Secondary color harmonies
- **Dynamic Range** → Brightness variation

#### Advanced Visual Effects
- Beat-driven chromatic aberration
- Onset-triggered color shifts
- Zero-crossing rate saturation effects
- Resolution-adaptive rendering
- Performance-based quality scaling

## Phase 5: Advanced Visual Effects and Optimization (Planned)

### Performance Optimization System 🔄
**Quality Scaling Modes:**
- **High**: Full effect processing at native resolution
- **Medium**: Reduced pattern complexity, optimized calculations
- **Low**: Simplified effects, basic color generation
- **Potato**: Fallback to current simple shader for low-end hardware

### User Control Interface 🔄
**Runtime Control Features:**
- Keyboard shortcuts for shader switching (1-8 keys)
- Automatic shader selection based on music characteristics
- Effect weight adjustment via controls
- Real-time parameter tweaking for experimentation

### Technical Architecture Enhancements 🔄
**Implementation Details:**
- LOD (Level of Detail) system for complex calculations
- GPU capability detection and fallback shaders
- Efficient uniform buffer updates with minimal GPU synchronization
- Shader compilation caching for faster startup
- Comprehensive error handling and graceful degradation

## Implementation Timeline

### Phase 4 Milestones (12 weeks)
- **Weeks 1-2**: Enhanced audio analysis implementation
- **Weeks 3-4**: Multi-shader architecture and uniform system
- **Weeks 5-6**: Parametric wave shader integration
- **Weeks 7-8**: Additional visual effect modes (Plasma, Kaleidoscope, Tunnel)
- **Weeks 9-10**: Advanced effect modes (Particle, Fractal, Spectralizer)
- **Weeks 11-12**: Performance optimization and quality scaling

### Phase 5 Goals (Future)
- Professional-grade visualization quality
- Intelligent adaptation to different music genres
- Real-time customization capabilities
- Optimal hardware utilization across different GPU capabilities

## Expected Transformation
Upon completion, Aruu will evolve from a sophisticated proof-of-concept into a professional-grade audio visualizer with:
- **Multiple intelligent visual modes** adapting to music characteristics
- **Advanced mathematical pattern generation** rivaling commercial software
- **Professional audio analysis pipeline** with 15+ parameters
- **Flexible, extensible architecture** for future enhancements