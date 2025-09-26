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

// Enhanced plasma noise functions with audio reactivity
fn noise(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn smooth_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f); // Smoothstep

    return mix(mix(noise(i + vec2<f32>(0.0, 0.0)),
                   noise(i + vec2<f32>(1.0, 0.0)), u.x),
               mix(noise(i + vec2<f32>(0.0, 1.0)),
                   noise(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}

fn fractal_noise(p: vec2<f32>, octaves: i32) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var pos = p;

    for (var i = 0; i < octaves; i = i + 1) {
        value += smooth_noise(pos) * amplitude;
        pos *= 2.0;
        amplitude *= 0.5;
    }

    return value;
}

// Plasma generation with audio-reactive parameters
fn generate_plasma(uv: vec2<f32>) -> f32 {
    let time = uniforms.time;

    // Base plasma layers driven by different frequency bands
    let bass_scale = 2.0 + uniforms.bass * 4.0;
    let mid_scale = 3.0 + uniforms.mid * 6.0;
    let treble_scale = 4.0 + uniforms.treble * 8.0;

    // BPM-synchronized flow
    let bpm_speed = uniforms.estimated_bpm / 120.0;
    let flow_speed = 1.0 + bpm_speed * 2.0;

    // Layer 1: Large-scale bass-driven flow
    let layer1 = fractal_noise(uv * bass_scale + vec2<f32>(time * flow_speed * 0.3, time * flow_speed * 0.2), 4);

    // Layer 2: Mid-frequency turbulence
    let layer2 = fractal_noise(uv * mid_scale + vec2<f32>(time * flow_speed * -0.4, time * flow_speed * 0.5), 3);

    // Layer 3: High-frequency details
    let layer3 = fractal_noise(uv * treble_scale + vec2<f32>(time * flow_speed * 0.6, time * flow_speed * -0.3), 2);

    // Beat-driven pulse modulation
    let beat_pulse = 1.0 + uniforms.beat_strength * sin(time * flow_speed * 8.0) * 0.3;

    // Onset creates sudden texture shifts
    let onset_distortion = uniforms.onset_strength * sin(uv.x * 20.0 + time * 10.0) * 0.1;

    // Spectral flux adds dynamic texture variation
    let flux_variation = uniforms.spectral_flux * fractal_noise(uv * 10.0 + vec2<f32>(time * 2.0), 2) * 0.2;

    // Combine layers with audio-reactive weights
    var plasma = layer1 * 0.5 + layer2 * 0.3 + layer3 * 0.2;
    plasma = plasma * beat_pulse + onset_distortion + flux_variation;

    // Dynamic range affects plasma contrast
    let contrast = 1.0 + uniforms.dynamic_range * 0.5;
    plasma = pow(abs(plasma), 1.0 / contrast) * sign(plasma);

    return plasma;
}

// Audio-reactive color generation for plasma
fn get_plasma_color(plasma: f32, uv: vec2<f32>) -> vec3<f32> {
    // Base hue shifts with spectral centroid
    let base_hue = uniforms.spectral_centroid * 0.0001 + uniforms.time * 0.1;

    // Plasma value affects hue variation
    let plasma_hue = base_hue + plasma * 0.3;

    // Frequency bands drive different color channels
    let bass_color = vec3<f32>(1.0, 0.3, 0.1) * uniforms.bass;
    let mid_color = vec3<f32>(0.2, 1.0, 0.4) * uniforms.mid;
    let treble_color = vec3<f32>(0.1, 0.4, 1.0) * uniforms.treble;

    // Combine frequency-based colors
    var freq_color = bass_color + mid_color + treble_color;
    freq_color = normalize(freq_color + vec3<f32>(0.1)); // Prevent zero vector

    // HSV color generation with audio-reactive parameters
    let saturation = uniforms.saturation * (0.7 + uniforms.pitch_confidence * 0.3);
    let brightness = uniforms.overall_volume * (0.5 + abs(plasma) * 0.5);

    var hsv_color = hsv_to_rgb(vec3<f32>(fract(plasma_hue), saturation, brightness));

    // Blend with frequency-based color
    let freq_blend = uniforms.spectral_flux * 0.5;
    hsv_color = mix(hsv_color, freq_color * brightness, freq_blend);

    // Beat-driven color pulsing
    let beat_boost = 1.0 + uniforms.beat_strength * sin(uniforms.time * 4.0) * 0.2;
    hsv_color = hsv_color * beat_boost;

    return hsv_color;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize coordinates to screen center
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    let uv = (in.tex_coords * 2.0 - 1.0) * vec2<f32>(resolution.x / resolution.y, 1.0);

    // Generate plasma field
    let plasma = generate_plasma(uv);

    // Generate audio-reactive color
    var color = get_plasma_color(plasma, uv);

    // Radial falloff with bass extension
    let radius = length(uv);
    let bass_extension = 1.0 + uniforms.bass * 0.4;
    let falloff_radius = 1.2 * bass_extension;
    let falloff = 1.0 - smoothstep(falloff_radius * 0.7, falloff_radius, radius);

    color = color * falloff;

    // Zero crossing rate affects overall texture sharpness
    let sharpness = 1.0 + uniforms.zero_crossing_rate * 0.3;
    color = pow(color, vec3<f32>(1.0 / sharpness));

    // Apply color intensity and ensure valid range
    color = color * uniforms.color_intensity;
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}