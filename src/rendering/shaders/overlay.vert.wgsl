// Overlay vertex shader - renders full-screen quads for GUI overlays

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) screen_pos: vec2<f32>,
}

// CRITICAL: This must match UniversalUniforms exactly in size and order
struct Uniforms {
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
    resolution_x: f32,
    resolution_y: f32,

    // Safety multipliers
    safety_beat_intensity: f32,
    safety_onset_intensity: f32,
    safety_color_change_rate: f32,
    safety_brightness_range: f32,
    safety_pattern_complexity: f32,
    safety_emergency_stop: f32,

    // Overlay system uniforms
    mouse_x: f32,
    mouse_y: f32,
    mouse_pressed: f32,
    show_debug_overlay: f32,
    show_control_panel: f32,
    ui_volume: f32,
    ui_is_playing: f32,
    ui_safety_level: f32,
    ui_quality_level: f32,
    ui_auto_shader: f32,
    ui_current_shader_index: f32,
    ui_fps: f32,
    ui_frame_time: f32,
    screen_width: f32,
    screen_height: f32,
    text_scale: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Pass through position for full-screen quad
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.tex_coords = input.tex_coords;

    // Screen position (0,0 to 1,1) for UI calculations
    output.screen_pos = input.tex_coords;

    return output;
}