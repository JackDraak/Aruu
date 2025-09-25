pub const VERTEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.world_position = model.position;
    return out;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
}

struct UniformData {
    color_intensity: f32,
    frequency_scale: f32,
    time_factor: f32,
    bass_response: f32,
    mid_response: f32,
    treble_response: f32,
    overall_brightness: f32,
    spectral_shift: f32,
    time: f32,
    _padding0: f32,
    _padding1: f32,
    _padding2: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: UniformData;

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
        uniforms.bass_response,
        uniforms.mid_response,
        uniforms.treble_response
    );

    let time_scaled = uniforms.time * uniforms.time_factor;

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

    // Dynamic color cycling based on audio
    let hue_base = (angle / 6.28318) + uniforms.spectral_shift + time_scaled * 0.05;
    let hue_modulation = combined_wave * 0.3;
    let final_hue = fract(hue_base + hue_modulation);

    // Enhanced saturation and brightness
    let saturation = uniforms.color_intensity * (0.7 + frequency_bands.y * 0.3);
    let brightness_base = uniforms.overall_brightness;
    let brightness_wave = (0.6 + combined_wave * 0.4);
    let final_brightness = brightness_base * brightness_wave;

    let color = hsv_to_rgb(vec3<f32>(final_hue, saturation, final_brightness));

    // Improved radial fade with bass response
    let bass_boost = 1.0 + frequency_bands.x * 0.3;
    let fade_distance = 0.9 * bass_boost;
    let fade = 1.0 - smoothstep(fade_distance, fade_distance + 0.3, distance_from_center);

    // Add center glow effect
    let center_glow = exp(-distance_from_center * 2.0) * uniforms.overall_brightness * 0.3;
    let final_color = color * fade + vec3<f32>(center_glow);

    return vec4<f32>(final_color, 1.0);
}
"#;