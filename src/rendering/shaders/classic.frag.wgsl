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

    // Padding
    _padding0: f32,
    _padding1: f32,
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

fn noise(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let uv = in.tex_coords * 2.0 - 1.0;
    let distance_from_center = length(uv);
    let angle = atan2(uv.y, uv.x);

    let frequency_bands = vec3<f32>(
        uniforms.bass,
        uniforms.mid,
        uniforms.treble
    );

    let time_scaled = uniforms.time * (1.0 + uniforms.overall_volume * 0.5);

    // Enhanced wave patterns with radial distortion
    let radial_freq = distance_from_center * 12.0 * uniforms.frequency_scale;
    let angular_freq = angle * 4.0 + time_scaled * 0.5;

    let bass_wave = sin(radial_freq + time_scaled) * frequency_bands.x * 0.8;
    let mid_wave = sin(radial_freq * 2.0 + angular_freq + time_scaled * 1.3) * frequency_bands.y * 0.6;
    let treble_wave = sin(radial_freq * 4.0 + angular_freq * 2.0 + time_scaled * 2.1) * frequency_bands.z * 0.4;

    // Add noise texture for high-frequency detail
    let noise_coord = uv * 20.0 + vec2<f32>(time_scaled * 0.1);
    let noise_val = noise(noise_coord) * frequency_bands.z * 0.1;

    let combined_wave = bass_wave + mid_wave + treble_wave + noise_val;

    // Cross-fade palette-based color generation
    var current_hue: f32;
    var current_saturation: f32;
    var prev_hue: f32;
    var prev_saturation: f32;

    // Calculate current palette color
    if uniforms.palette_index < 0.5 {
        // Rainbow palette (index 0)
        current_hue = fract((angle / 6.28318) + time_scaled * 0.05 + combined_wave * 0.3);
        current_saturation = uniforms.saturation * uniforms.color_intensity * (0.8 + frequency_bands.y * 0.2);
    } else {
        // Hue-based palettes (index 1-7)
        let hue_variation = combined_wave * uniforms.palette_hue_range * 0.5;
        current_hue = fract(uniforms.palette_base_hue + hue_variation);
        current_saturation = uniforms.saturation * uniforms.color_intensity * 0.9;
    }

    // Calculate previous palette color
    if uniforms.prev_palette_index < 0.5 {
        // Rainbow palette (index 0)
        prev_hue = fract((angle / 6.28318) + time_scaled * 0.05 + combined_wave * 0.3);
        prev_saturation = uniforms.saturation * uniforms.color_intensity * (0.8 + frequency_bands.y * 0.2);
    } else {
        // Hue-based palettes (index 1-7)
        let prev_hue_variation = combined_wave * uniforms.prev_palette_hue_range * 0.5;
        prev_hue = fract(uniforms.prev_palette_base_hue + prev_hue_variation);
        prev_saturation = uniforms.saturation * uniforms.color_intensity * 0.9;
    }

    // Blend between previous and current palettes
    let final_hue = mix(prev_hue, current_hue, uniforms.transition_blend);
    let final_saturation = mix(prev_saturation, current_saturation, uniforms.transition_blend);

    let brightness_base = uniforms.overall_volume;
    let brightness_wave = (0.6 + combined_wave * 0.4);
    let final_brightness = brightness_base * brightness_wave;

    let color = hsv_to_rgb(vec3<f32>(final_hue, final_saturation, final_brightness));

    // Improved radial fade with bass response
    let bass_boost = 1.0 + frequency_bands.x * 0.3;
    let fade_distance = 0.9 * bass_boost;
    let fade = 1.0 - smoothstep(fade_distance, fade_distance + 0.3, distance_from_center);

    // Add center glow effect
    let center_glow = exp(-distance_from_center * 2.0) * uniforms.overall_volume * 0.3;
    let final_color = color * fade + vec3<f32>(center_glow);

    return vec4<f32>(final_color, 1.0);
}