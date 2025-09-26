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
}

@group(0) @binding(0)
var<uniform> uniforms: UniversalUniforms;

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

// Kaleidoscope symmetry functions
fn kaleidoscope_fold(uv: vec2<f32>, segments: f32) -> vec2<f32> {
    let angle = atan2(uv.y, uv.x);
    let radius = length(uv);

    // Fold angle into single segment
    let segment_angle = 2.0 * 3.14159 / segments;
    let folded_angle = abs(fract(angle / segment_angle + 0.5) - 0.5) * segment_angle;

    return vec2<f32>(cos(folded_angle), sin(folded_angle)) * radius;
}

// Generate pattern within a single kaleidoscope segment
fn generate_segment_pattern(uv: vec2<f32>) -> f32 {
    let time = uniforms.time;

    // BPM-synchronized rotation
    let bpm_speed = uniforms.estimated_bpm / 120.0;
    let rotation_speed = 0.5 + bpm_speed * 1.0;
    let rotation = time * rotation_speed;

    // Apply rotation
    let cos_r = cos(rotation);
    let sin_r = sin(rotation);
    let rotated_uv = vec2<f32>(
        uv.x * cos_r - uv.y * sin_r,
        uv.x * sin_r + uv.y * cos_r
    );

    let radius = length(rotated_uv);
    let angle = atan2(rotated_uv.y, rotated_uv.x);

    // Multiple concentric patterns driven by frequency bands
    let bass_rings = sin(radius * (5.0 + uniforms.bass * 15.0) - time * 2.0);
    let mid_spirals = sin(angle * (6.0 + uniforms.mid * 12.0) + radius * 8.0 - time * 3.0);
    let treble_texture = sin(rotated_uv.x * (20.0 + uniforms.treble * 30.0)) *
                         cos(rotated_uv.y * (25.0 + uniforms.presence * 20.0));

    // Beat-driven radial pulses
    let beat_pulse = 1.0 + uniforms.beat_strength * sin(time * 8.0) * 0.5;
    let radial_pulse = sin(radius * 10.0 * beat_pulse - time * 4.0);

    // Pitch confidence creates harmonic patterns
    let harmonic_pattern = sin(angle * (4.0 + uniforms.pitch_confidence * 8.0)) *
                          cos(radius * (3.0 + uniforms.pitch_confidence * 6.0));

    // Onset strength creates sudden pattern shifts
    let onset_shift = uniforms.onset_strength * sin(rotated_uv.x * 30.0 + time * 20.0) * 0.3;

    // Combine all patterns
    var pattern = (bass_rings * 0.4 + mid_spirals * 0.3 + treble_texture * 0.2 +
                   radial_pulse * 0.3 + harmonic_pattern * 0.4) * uniforms.overall_volume;

    pattern += onset_shift;

    // Dynamic range affects pattern contrast
    let contrast = 1.0 + uniforms.dynamic_range * 0.5;
    pattern = pow(abs(pattern), 1.0 / contrast) * sign(pattern);

    return pattern;
}

// Audio-reactive color generation for kaleidoscope
fn get_kaleidoscope_color(pattern: f32, uv: vec2<f32>) -> vec3<f32> {
    let angle = atan2(uv.y, uv.x);

    // Base hue rotates with spectral centroid
    let base_hue = uniforms.spectral_centroid * 0.0001 + uniforms.time * 0.05;

    // Pattern value affects hue shifts
    let pattern_hue = base_hue + pattern * 0.2 + angle / 6.28318;

    // Frequency bands create color harmonies
    let bass_influence = uniforms.bass * vec3<f32>(1.0, 0.2, 0.1);
    let mid_influence = uniforms.mid * vec3<f32>(0.3, 1.0, 0.3);
    let treble_influence = uniforms.treble * vec3<f32>(0.1, 0.3, 1.0);

    // Pitch confidence enhances color harmony
    let harmony_factor = uniforms.pitch_confidence;
    let harmonic_hue = base_hue + harmony_factor * sin(pattern * 3.0) * 0.3;

    // Generate base color
    let saturation = uniforms.saturation * (0.8 + harmony_factor * 0.2);
    let brightness = uniforms.overall_volume * (0.6 + abs(pattern) * 0.4);

    var color = hsv_to_rgb(vec3<f32>(fract(harmonic_hue), saturation, brightness));

    // Blend with frequency influences
    let freq_color = bass_influence + mid_influence + treble_influence;
    let freq_blend = uniforms.spectral_flux * 0.4;
    color = mix(color, freq_color * brightness, freq_blend);

    // Beat-driven brightness pulses
    let beat_boost = 1.0 + uniforms.beat_strength * sin(uniforms.time * 6.0) * 0.3;
    color = color * beat_boost;

    // Zero crossing rate affects color vibrancy
    let vibrancy = 1.0 + uniforms.zero_crossing_rate * 0.2;
    color = pow(color, vec3<f32>(1.0 / vibrancy));

    return color;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize coordinates to screen center
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    let uv = (in.tex_coords * 2.0 - 1.0) * vec2<f32>(resolution.x / resolution.y, 1.0);

    // Dynamic segment count based on pitch confidence and rhythm
    let base_segments = 6.0;
    let pitch_segments = uniforms.pitch_confidence * 6.0;
    let rhythm_segments = uniforms.tempo_confidence * 2.0;
    let segments = base_segments + pitch_segments + rhythm_segments;

    // Apply kaleidoscope folding
    let folded_uv = kaleidoscope_fold(uv, segments);

    // Generate pattern in the folded space
    let pattern = generate_segment_pattern(folded_uv);

    // Generate audio-reactive color
    var color = get_kaleidoscope_color(pattern, folded_uv);

    // Radial gradient with bass extension
    let radius = length(uv);
    let bass_extension = 1.0 + uniforms.bass * 0.3;
    let gradient_power = 0.8 + uniforms.sub_bass * 0.4;
    let gradient = 1.0 - pow(radius / bass_extension, gradient_power);

    color = color * max(gradient, 0.0);

    // Apply global effects
    color = color * uniforms.color_intensity;

    // Spectral flux creates subtle shimmer
    let shimmer = 1.0 + uniforms.spectral_flux * sin(uv.x * 50.0 + uniforms.time * 10.0) * 0.05;
    color = color * shimmer;

    // Ensure color values stay in valid range
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}