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

struct ShaderParams {
    color_intensity: f32,
    frequency_scale: f32,
    time_factor: f32,
    bass_response: f32,
    mid_response: f32,
    treble_response: f32,
    overall_brightness: f32,
    spectral_shift: f32,
}

@group(0) @binding(0)
var<uniform> params: ShaderParams;

@group(0) @binding(1)
var<uniform> time: f32;

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

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let uv = in.tex_coords * 2.0 - 1.0;
    let distance_from_center = length(uv);

    let frequency_bands = vec3<f32>(
        params.bass_response,
        params.mid_response,
        params.treble_response
    );

    let time_scaled = time * params.time_factor;

    let wave1 = sin(distance_from_center * 8.0 * params.frequency_scale + time_scaled) * frequency_bands.x;
    let wave2 = sin(distance_from_center * 16.0 * params.frequency_scale + time_scaled * 1.5) * frequency_bands.y;
    let wave3 = sin(distance_from_center * 32.0 * params.frequency_scale + time_scaled * 2.0) * frequency_bands.z;

    let combined_wave = (wave1 + wave2 + wave3) * 0.33;

    let hue = (combined_wave + params.spectral_shift + time_scaled * 0.1) % 1.0;
    let saturation = params.color_intensity * 0.8 + 0.2;
    let brightness = params.overall_brightness * (0.5 + combined_wave * 0.5);

    let color = hsv_to_rgb(vec3<f32>(hue, saturation, brightness));

    let fade = 1.0 - smoothstep(0.8, 1.2, distance_from_center);

    return vec4<f32>(color * fade, 1.0);
}
"#;