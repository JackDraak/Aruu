struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
}

struct UniversalUniforms {
    // 5-band frequency analysis
    sub_bass: f32,
    bass: f32,
    mid: f32,
    treble: f32,
    presence: f32,

    // Volume and dynamics
    overall_volume: f32,
    signal_level_db: f32,
    peak_level_db: f32,
    dynamic_range: f32,

    // Enhanced rhythm analysis
    beat_strength: f32,
    estimated_bpm: f32,
    tempo_confidence: f32,
    onset_detected: f32,
    downbeat_detected: f32,

    // Spectral characteristics
    spectral_centroid: f32,
    spectral_rolloff: f32,
    spectral_flux: f32,
    pitch_confidence: f32,
    zero_crossing_rate: f32,
    onset_strength: f32,

    // Visual controls
    time: f32,
    color_intensity: f32,
    frequency_scale: f32,
    saturation: f32,
    palette_index: f32,
    palette_base_hue: f32,
    palette_hue_range: f32,
    transition_blend: f32,
    prev_palette_index: f32,
    prev_palette_base_hue: f32,
    prev_palette_hue_range: f32,

    // Effect weights
    plasma_weight: f32,
    kaleidoscope_weight: f32,
    tunnel_weight: f32,
    particle_weight: f32,
    fractal_weight: f32,
    spectralizer_weight: f32,

    // System parameters
    projection_mode: f32,
    smoothing_factor: f32,

    // Resolution
    resolution_x: f32,
    resolution_y: f32,

    // Safety multipliers for epilepsy prevention
    safety_beat_intensity: f32,
    safety_onset_intensity: f32,
    safety_color_change_rate: f32,
    safety_brightness_range: f32,
    safety_pattern_complexity: f32,
    safety_emergency_stop: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: UniversalUniforms;

// Audio-reactive parameters derived from analysis
struct AudioParams {
    color1: vec3<f32>,      // Primary color from palette
    color2: vec3<f32>,      // Secondary color from palette
    frequency: f32,         // Pattern frequency from spectral data
    speed: f32,             // Animation speed from BPM
    intensity: f32,         // Overall intensity from volume/beat
}

fn hue_to_rgb(h: f32) -> vec3<f32> {
    let c = vec3<f32>(abs(h * 6.0 - 3.0) - 1.0,
                      2.0 - abs(h * 6.0 - 2.0),
                      2.0 - abs(h * 6.0 - 4.0));
    return clamp(c, vec3<f32>(0.0), vec3<f32>(1.0));
}

fn hsv_to_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let rgb = hue_to_rgb(hsv.x);
    return ((rgb - 1.0) * hsv.y + 1.0) * hsv.z;
}

// Extract audio-reactive parameters
fn get_audio_params() -> AudioParams {
    var params: AudioParams;

    // Dynamic color selection based on frequency content
    let bass_dominance = uniforms.bass + uniforms.sub_bass;
    let mid_dominance = uniforms.mid;
    let treble_dominance = uniforms.treble + uniforms.presence;

    // Primary color shifts based on dominant frequency range
    if (bass_dominance > mid_dominance && bass_dominance > treble_dominance) {
        // Bass-heavy: warm colors (reds, oranges)
        params.color1 = vec3<f32>(1.0, 0.3 + bass_dominance * 0.7, 0.1);
    } else if (treble_dominance > bass_dominance && treble_dominance > mid_dominance) {
        // Treble-heavy: cool colors (blues, cyans)
        params.color1 = vec3<f32>(0.1, 0.3 + treble_dominance * 0.7, 1.0);
    } else {
        // Mid-heavy: green/yellow spectrum
        params.color1 = vec3<f32>(0.3 + mid_dominance * 0.7, 1.0, 0.2);
    }

    // Secondary color based on pitch confidence and spectral characteristics
    let harmonic_factor = uniforms.pitch_confidence;
    params.color2 = vec3<f32>(
        0.5 + harmonic_factor * 0.5,
        0.2 + uniforms.spectral_flux * 2.0,
        0.8 - uniforms.zero_crossing_rate * 0.6
    );

    // Pattern frequency driven by spectral centroid (brightness)
    params.frequency = 2.0 + uniforms.spectral_centroid * 0.0001 + uniforms.onset_strength * 5.0;

    // Animation speed synchronized to BPM
    let bpm_factor = uniforms.estimated_bpm / 120.0; // Normalize around 120 BPM
    params.speed = 1.0 + bpm_factor * 2.0 + uniforms.beat_strength * 3.0;

    // Intensity responds to volume and dynamic range
    params.intensity = 0.3 + uniforms.overall_volume * 0.7 + uniforms.dynamic_range * 0.5;

    return params;
}

// Enhanced pattern generation with audio responsiveness
fn generate_pattern(uv: vec2<f32>, params: AudioParams) -> f32 {
    let angle = atan2(uv.y, uv.x);
    let radius = length(uv);

    // Multiple wave patterns that respond to different audio features
    let wave1 = sin(radius * params.frequency - uniforms.time * params.speed);
    let wave2 = cos(angle * (4.0 + uniforms.bass * 8.0) + uniforms.time * params.speed * 0.7);
    let wave3 = sin(length(uv * (2.0 + uniforms.treble * 3.0)) * 3.0 - uniforms.time * params.speed * 1.3);

    // Beat-driven pulse waves
    // Safe beat pulse with safety multipliers
    let safe_beat_strength = uniforms.beat_strength * uniforms.safety_beat_intensity;
    let beat_pulse = sin(uniforms.time * params.speed * 2.0) * safe_beat_strength * 0.5; // Slower, gentler
    let wave4 = cos(radius * 8.0 + beat_pulse * 5.0); // Reduced intensity

    // Spectral flux creates texture variation
    let texture_noise = sin(uv.x * 20.0 + uniforms.spectral_flux * 50.0) *
                       cos(uv.y * 15.0 + uniforms.spectral_flux * 30.0) * 0.1;

    // Combine all patterns with intensity control
    return (wave1 + wave2 + wave3 + wave4 + texture_noise) * params.intensity;
}

// Advanced color blending with audio-reactive chromatic effects
fn apply_audio_chromatic_effects(color: vec3<f32>, pattern: f32) -> vec3<f32> {
    var enhanced_color = color;

    // Safe chromatic aberration with safety multipliers
    let safe_beat_effect = uniforms.beat_strength * uniforms.safety_beat_intensity;
    enhanced_color.r = enhanced_color.r + sin(uniforms.time * 0.3 + safe_beat_effect * 2.0) * 0.05; // Reduced intensity
    enhanced_color.b = enhanced_color.b + cos(uniforms.time * 0.2 + safe_beat_effect * 1.5) * 0.05; // Reduced intensity

    // Safe onset shifts with gradual transitions
    let safe_onset_strength = uniforms.onset_strength * uniforms.safety_onset_intensity;
    let onset_shift = safe_onset_strength * sin(pattern * 3.0) * 0.1; // Reduced from 0.2
    enhanced_color.g = enhanced_color.g + onset_shift;

    // Zero crossing rate affects color saturation
    let saturation_factor = 1.0 + uniforms.zero_crossing_rate * 0.5;
    enhanced_color = enhanced_color * saturation_factor;

    return enhanced_color;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize coordinates to screen center
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    let uv = (in.tex_coords * 2.0 - 1.0) * vec2<f32>(resolution.x / resolution.y, 1.0);

    // Get audio-reactive parameters
    let params = get_audio_params();

    // Generate complex pattern
    let pattern = generate_pattern(uv, params);

    // Create dynamic color palette based on pattern and audio
    var color = vec3<f32>(
        params.color1.r * (0.5 + 0.5 * cos(pattern)),
        params.color1.g * (0.5 + 0.5 * sin(pattern * 1.5)),
        params.color2.b * (0.5 + 0.5 * cos(pattern * 2.0))
    );

    // Apply audio-driven chromatic effects
    color = apply_audio_chromatic_effects(color, pattern);

    // Safe radial gradient with controlled bass response
    let radius = length(uv);
    let safe_bass_effect = uniforms.bass * uniforms.safety_beat_intensity;
    let gradient_power = 0.7 + safe_bass_effect * 0.3; // Reduced bass influence
    let gradient = 1.0 - pow(radius, gradient_power);
    color = color * gradient;

    // Safe dynamic range with brightness limits
    let safe_dynamic_factor = uniforms.dynamic_range * uniforms.safety_brightness_range;
    let brightness_factor = 0.8 + safe_dynamic_factor * 0.2; // Reduced brightness variation
    color = color * brightness_factor;

    // Apply global saturation control
    let final_saturation = uniforms.saturation;
    let luminance = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(vec3<f32>(luminance), color, final_saturation);

    // Apply safety brightness limits
    color = color * uniforms.safety_brightness_range;

    // Apply emergency stop override
    color = color * uniforms.safety_emergency_stop;

    // Emergency stop fallback: show dim gray
    if (uniforms.safety_emergency_stop < 0.1) {
        color = vec3<f32>(0.1, 0.1, 0.1);
    }

    // Ensure color values stay in valid range
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}