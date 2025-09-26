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
4. **Phase 4**: Multi-shader architecture ✅
5. **Phase 5**: System integration and optimization (In Progress)

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
- ✅ All tests passing (34 lib tests)
- ✅ Build successful with complete audio-visual pipeline
- ✅ Phase 4 Complete: Multi-shader architecture with 8 shader modes
- ✅ Enhanced audio analysis with 15+ parameters implemented
- ✅ Dynamic resolution support and performance optimization system

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

### Basic Usage
```bash
# Real-time microphone visualization with multi-shader system
cargo run

# Play audio file with visualization (supports WAV, M4A/AAC)
cargo run sample_gentle.wav
cargo run sample_rock.m4a

# Run simple multi-shader example
cargo run --example simple_multi_shader

# Run shader demonstration with auto-cycling
cargo run --example shader_demo sample.wav

# Run tests (34 lib tests)
cargo test --lib
```

### Interactive Controls
Once the visualizer is running, use these keyboard controls:

#### Shader Selection
- **1-8 Keys**: Direct shader selection
  - `1` - Classic: Enhanced traditional wave patterns
  - `2` - ParametricWave: Mathematical sine/cosine patterns
  - `3` - Plasma: Fluid organic patterns driven by low frequencies
  - `4` - Kaleidoscope: Symmetric patterns for harmonic content
  - `5` - Tunnel: 3D perspective effects with bass-driven depth
  - `6` - Particle: Dynamic particle systems for transients
  - `7` - Fractal: Mandelbrot/Julia sets scaled by spectral characteristics
  - `8` - Spectralizer: Direct frequency visualization with artistic flair

#### Advanced Controls
- **Space**: Cycle to next shader mode
- **A**: Toggle intelligent auto-shader selection (recommended!)
- **Q**: Cycle through quality levels (Ultra → High → Medium → Low → Potato → Auto)
- **P**: Toggle real-time performance overlay
- **H**: Display help and current status

### Intelligent Auto-Shader Selection
When auto-shader mode is enabled (press `A`), the system automatically selects the optimal shader based on real-time audio analysis:

- **Bass-Heavy Music** (electronic, hip-hop) → Classic or Tunnel shaders
- **Treble-Heavy + Transients** (rock, percussion) → Particle shader
- **Harmonic Content** (classical, vocals) → Kaleidoscope shader
- **High Spectral Flux** (dynamic changes) → ParametricWave shader
- **High Dynamic Range** (orchestral, jazz) → Fractal shader
- **Frequency-Rich Content** → Spectralizer shader

### Performance Optimization
The system includes adaptive performance scaling:

- **Ultra**: Maximum visual quality, all effects enabled
- **High**: Full resolution with all pattern complexity
- **Medium**: Optimized calculations, reduced complexity
- **Low**: Simplified effects for older hardware
- **Potato**: Minimal effects for very low-end systems
- **Auto**: Automatically adjusts based on real-time performance

### Audio Analysis Features
The enhanced system provides professional-grade audio analysis:

#### 5-Band Frequency Analysis
- **Sub-Bass** (20-60 Hz): Deep low-end impact
- **Bass** (60-200 Hz): Fundamental rhythmic content
- **Mid** (200-2000 Hz): Vocal and instrument fundamentals
- **Treble** (2000-8000 Hz): Clarity and presence
- **Presence** (8000+ Hz): Air and sparkle

#### Advanced Spectral Features
- **Spectral Flux**: Frame-to-frame changes for texture variation
- **Onset Strength**: Transient detection for visual bursts
- **Pitch Confidence**: Harmonic analysis for color coordination
- **Dynamic Range**: Volume variation over time
- **Zero Crossing Rate**: Signal complexity analysis
- **BPM Estimation**: Tempo detection with confidence metrics

## Audio Format Support
- **WAV**: PCM audio files
- **M4A/AAC**: Advanced Audio Coding files
- **Real-time**: Microphone input with CPAL

## Tips for Best Experience

### Recommended Audio Sources
- **Electronic/EDM**: Try Tunnel or Particle shaders for bass response
- **Classical/Orchestral**: Kaleidoscope or Fractal shaders showcase harmonic content
- **Rock/Metal**: Particle shader responds well to transients and dynamics
- **Jazz**: Fractal shader adapts to dynamic range variations
- **Ambient**: Plasma shader creates fluid organic patterns

### Performance Tips
- **High-End GPUs**: Use Ultra quality for maximum visual impact
- **Mid-Range Hardware**: High or Medium quality provides excellent balance
- **Older Systems**: Enable Auto quality for adaptive performance
- **Performance Issues**: Press `P` to monitor FPS, use `Q` to reduce quality

### Audio Input Tips
- **Microphone**: Ensure microphone permissions are granted
- **Audio Files**: Supported formats include WAV, M4A/AAC
- **Volume Levels**: Moderate volume levels work best for analysis
- **Silence Detection**: System gracefully handles silence and low-volume audio

## Troubleshooting

### Common Issues
- **No Audio Input**: System falls back to silent mode, visualization still works
- **Low Performance**: Use Quality controls (`Q` key) to optimize for your hardware
- **Window Size**: Visualizer adapts to any resolution automatically
- **Shader Issues**: All shaders include fallback modes for compatibility

### System Requirements
- **Minimum**: Any GPU with basic WGPU support
- **Recommended**: Dedicated GPU for Ultra quality mode
- **Audio**: Any audio input device or supported audio files
- **OS**: Windows, macOS, Linux (via CPAL and winit)

## Smoothing System Usage
```rust
// Configure custom smoothing for specific parameters
let mut mapper = FeatureMapper::new();
mapper.configure_smoothing("bass_response", SmoothingType::linear(0.8));
mapper.configure_smoothing("brightness", SmoothingType::exponential(5.0));
mapper.configure_smoothing("color", SmoothingType::adaptive(0.1, 0.7, 3.0));
```

## Phase 4: Multi-Shader Architecture Integration ✅

### Enhanced Audio Analysis Pipeline ✅
**Professional-grade audio analysis with 15+ parameters**

#### 5-Band Frequency Analysis ✅
- **Sub-Bass**: 20-60 Hz (deep low-end content)
- **Bass**: 60-200 Hz (fundamental bass frequencies)
- **Mid**: 200-2000 Hz (vocal and instrument fundamentals)
- **Treble**: 2000-8000 Hz (clarity and presence)
- **Presence**: 8000+ Hz (air and sparkle)

#### Advanced Spectral Features ✅
- **Spectral Flux**: Frame-to-frame spectral difference for texture variation
- **Onset Strength**: Transient detection for sudden visual changes
- **Pitch Confidence**: Harmonic content analysis for color harmonies
- **Dynamic Range**: RMS variation over time windows
- **Zero Crossing Rate**: Signal complexity analysis
- **Enhanced BPM**: Histogram and autocorrelation-based tempo estimation

#### Advanced Audio Analyzer ✅
- **Stateful Analysis**: `AdvancedAudioAnalyzer` for temporal feature tracking
- **Spectral Flux Calculation**: Frame-to-frame spectral change detection
- **Dynamic Range Tracking**: RMS variation analysis over time windows
- **Enhanced Feature Pipeline**: Integration with existing FFT and rhythm systems

### Multi-Shader Architecture ✅
**Flexible shader system supporting multiple visual modes**

#### Core Shader System Components ✅
- **ShaderSystem**: Manages 8 shader modes with hot-swapping capability
- **UniversalUniforms**: 40+ parameter unified buffer for all shaders
- **PerformanceManager**: Quality scaling with 5 performance levels
- **EnhancedFrameComposer**: Intelligent rendering with performance monitoring

#### Available Shader Modes ✅
1. **Classic Mode**: Enhanced version with new audio parameters
2. **ParametricWave**: Mathematical sine/cosine patterns with audio modulation
3. **Plasma**: Fluid organic patterns driven by low frequencies
4. **Kaleidoscope**: Symmetric patterns responding to harmonic content
5. **Tunnel**: 3D perspective effects with bass-driven depth
6. **Particle**: Dynamic particle systems responding to transients
7. **Fractal**: Mandelbrot/Julia sets scaled by spectral characteristics
8. **Spectralizer**: Frequency visualization with bars, waveforms, and particles

### Intelligent Audio-Visual Systems ✅

#### Dynamic Resolution Support ✅
- All shaders use runtime resolution detection
- Aspect ratio correction for different screen sizes
- Performance scaling based on resolution

#### Intelligent Shader Selection ✅
- **Auto-selection algorithm** based on audio characteristics:
  - Bass-heavy → Classic/Tunnel modes
  - Treble-heavy + onsets → Particle mode
  - Harmonic content → Kaleidoscope mode
  - High spectral flux → ParametricWave mode
  - High dynamic range → Fractal mode
- **Manual control** via keyboard shortcuts (1-8 keys)

#### Advanced Visual Effects ✅
- Beat-driven amplitude modulation across all modes
- Onset-triggered visual bursts and color flashes
- BPM-synchronized movement and evolution
- Spectral flux texture variation
- Zero-crossing rate affecting visual complexity
- Performance-adaptive quality scaling

## Phase 5: System Integration and Optimization (In Progress)

### Performance Optimization System ✅ (Implemented in Phase 4)
**Quality Scaling Modes:**
- **Ultra**: Maximum visual quality with all effects
- **High**: Full effect processing at native resolution
- **Medium**: Reduced pattern complexity, optimized calculations
- **Low**: Simplified effects, basic color generation
- **Potato**: Fallback performance mode for low-end hardware

### User Control Interface ✅ (Implemented in Phase 4)
**Runtime Control Features:**
- **Keyboard shortcuts** for shader switching (1-8 keys)
- **Automatic shader selection** based on music characteristics
- **Quality override controls** (Q key for manual quality adjustment)
- **Auto-shader toggle** (A key to enable/disable intelligent selection)
- **Performance overlay** (P key for real-time metrics display)
- **Help system** (H key for control reference)

### Integration Tasks 🔄
**Current Focus:**
- **Main Visualizer Integration**: Update core visualizer to use multi-shader system
- **API Modernization**: Update examples to use new enhanced APIs
- **System Integration**: Seamless integration between all components
- **User Documentation**: Comprehensive usage and control documentation

### Technical Architecture Status ✅
**Implemented Features:**
- **Unified uniform system** with 40+ parameters across all shaders
- **Efficient GPU synchronization** with minimal buffer updates
- **Performance monitoring** with real-time FPS and timing metrics
- **Quality adaptation** based on hardware performance
- **Graceful error handling** with shader compilation fallbacks

## Implementation Timeline

### Phase 4 Milestones ✅ (Completed)
- ✅ Enhanced audio analysis implementation (5-band analysis, spectral flux, dynamic range)
- ✅ Multi-shader architecture and uniform system (8 shader modes, 40+ parameters)
- ✅ Advanced visual effect modes (Classic, ParametricWave, Plasma, Kaleidoscope)
- ✅ Complex effect modes (Tunnel, Particle, Fractal, Spectralizer)
- ✅ Performance optimization and quality scaling (5-level quality system)
- ✅ User control interface with intelligent auto-selection

### Phase 5 Goals ✅ (Completed)
- ✅ **System Integration**: Seamlessly integrated multi-shader system with main visualizer
- ✅ **API Modernization**: Updated examples and comprehensive documentation
- ✅ **User Experience**: Polished controls with comprehensive user documentation
- ✅ **Performance Optimization**: Fine-tuned for optimal hardware utilization

## Developer Information

### Architecture Overview
The Aruu system consists of three main layers:

#### Audio Processing Layer
- **AudioProcessor**: Core audio input/output with CPAL integration
- **FftAnalyzer**: Real-time FFT analysis with Hann windowing
- **AdvancedAudioAnalyzer**: Temporal audio analysis for spectral features
- **RhythmDetector**: Enhanced rhythm and tempo detection
- **AudioFeatures**: Unified audio feature representation (15+ parameters)

#### Rendering Layer
- **WgpuContext**: GPU context management with winit integration
- **ShaderSystem**: Hot-swappable shader management with 8 modes
- **EnhancedFrameComposer**: Intelligent rendering with performance monitoring
- **PerformanceManager**: Adaptive quality scaling system
- **UniversalUniforms**: 40+ parameter unified buffer for all shaders

#### Control Layer
- **UserInterface**: Centralized keyboard input handling
- **FeatureMapper**: Legacy parameter mapping with advanced smoothing
- **PaletteManager**: Dynamic color palette management
- **Smoother**: Advanced smoothing algorithms (Linear/Exponential/Adaptive)

### Extending the System

#### Adding New Shaders
1. Create new shader variant in `ShaderType` enum
2. Implement WGSL fragment shader using `UniversalUniforms`
3. Add to `ShaderSystem::available_shaders()`
4. Update intelligent selection algorithm in `EnhancedFrameComposer`

#### Adding Audio Features
1. Extend `AudioFeatures` struct with new parameters
2. Implement analysis in `AdvancedAudioAnalyzer`
3. Add to `UniversalUniforms` structure
4. Update shader mapping in `ShaderSystem::map_audio_data()`

#### Performance Optimization
1. Add quality levels in `QualityLevel` enum
2. Implement quality-specific rendering in shaders
3. Update `PerformanceManager::should_adjust_quality()`
4. Add performance metrics tracking

## Expected Transformation
Upon completion, Aruu will evolve from a sophisticated proof-of-concept into a professional-grade audio visualizer with:
- **Multiple intelligent visual modes** adapting to music characteristics
- **Advanced mathematical pattern generation** rivaling commercial software
- **Professional audio analysis pipeline** with 15+ parameters
- **Flexible, extensible architecture** for future enhancements