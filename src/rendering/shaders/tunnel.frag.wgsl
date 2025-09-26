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

// 3D tunnel perspective transformation
fn tunnel_coordinates(uv: vec2<f32>) -> vec3<f32> {
    let radius = length(uv);
    let angle = atan2(uv.y, uv.x);

    // Prevent division by zero
    let safe_radius = max(radius, 0.01);

    // Perspective depth calculation - bass extends the tunnel
    let bass_depth_factor = 1.0 + uniforms.bass * 2.0 + uniforms.sub_bass * 1.5;
    let depth = 1.0 / safe_radius * bass_depth_factor;

    // Beat-driven depth pulsing
    let beat_pulse = 1.0 + uniforms.beat_strength * sin(uniforms.time * 8.0) * 0.3;
    let final_depth = depth * beat_pulse;

    return vec3<f32>(angle, final_depth, radius);
}

// Generate tunnel rings and radial patterns
fn generate_tunnel_pattern(tunnel_coord: vec3<f32>) -> f32 {
    let angle = tunnel_coord.x;
    let depth = tunnel_coord.y;
    let radius = tunnel_coord.z;

    let time = uniforms.time;

    // BPM-synchronized movement through tunnel
    let bpm_speed = uniforms.estimated_bpm / 120.0;
    let tunnel_speed = 1.0 + bpm_speed * 2.0;
    let movement = time * tunnel_speed;

    // Concentric rings driven by bass
    let ring_frequency = 5.0 + uniforms.bass * 15.0;
    let rings = sin(depth * ring_frequency - movement * 3.0);

    // Radial spokes driven by mid frequencies
    let spoke_count = 8.0 + uniforms.mid * 16.0;
    let spokes = sin(angle * spoke_count + depth * 2.0 - movement * 2.0);

    // High-frequency texture on tunnel walls
    let wall_texture = sin(depth * (30.0 + uniforms.treble * 50.0) +
                          sin(angle * (20.0 + uniforms.presence * 30.0)) * 0.5);

    // Beat-driven radial bursts
    let burst_pattern = sin(angle * 4.0 + uniforms.beat_strength * 10.0) *
                       exp(-radius * (2.0 - uniforms.beat_strength));

    // Onset creates sudden depth shifts
    let onset_shift = uniforms.onset_strength * sin(depth * 20.0 + time * 15.0) * 0.3;

    // Pitch confidence creates harmonic tunnel segments
    let harmonic_segments = sin(depth * (3.0 + uniforms.pitch_confidence * 6.0)) *
                           cos(angle * (2.0 + uniforms.pitch_confidence * 4.0));

    // Spectral flux adds dynamic texture variation along tunnel
    let flux_variation = uniforms.spectral_flux * sin(depth * 25.0 + angle * 10.0 + time * 5.0) * 0.2;

    // Combine all patterns
    var pattern = rings * 0.4 + spokes * 0.3 + wall_texture * 0.15 +
                  burst_pattern * 0.25 + harmonic_segments * 0.2;

    pattern += onset_shift + flux_variation;

    // Dynamic range affects pattern intensity
    pattern = pattern * (0.7 + uniforms.dynamic_range * 0.6);

    // Distance-based attenuation for depth perception
    let distance_fade = exp(-radius * 1.5);
    pattern = pattern * distance_fade;

    return pattern;
}

// Audio-reactive color generation for tunnel
fn get_tunnel_color(pattern: f32, tunnel_coord: vec3<f32>) -> vec3<f32> {
    let depth = tunnel_coord.y;
    let angle = tunnel_coord.x;

    // Base hue shifts with movement through tunnel
    let base_hue = uniforms.time * 0.1 + depth * 0.05;

    // Spectral centroid affects hue modulation
    let spectral_hue = base_hue + uniforms.spectral_centroid * 0.0001;

    // Depth-based color progression
    let depth_hue = spectral_hue + depth * 0.1 + pattern * 0.2;

    // Frequency-based color zones in tunnel
    let bass_zone = smoothstep(0.0, 2.0, depth) * uniforms.bass;
    let mid_zone = smoothstep(1.0, 4.0, depth) * uniforms.mid;
    let treble_zone = smoothstep(3.0, 8.0, depth) * uniforms.treble;

    // Color influence by frequency zones
    let bass_color = vec3<f32>(1.0, 0.3, 0.1) * bass_zone;
    let mid_color = vec3<f32>(0.2, 1.0, 0.4) * mid_zone;
    let treble_color = vec3<f32>(0.1, 0.4, 1.0) * treble_zone;

    // HSV color generation
    let saturation = uniforms.saturation * (0.8 + uniforms.tempo_confidence * 0.2);
    let brightness = uniforms.overall_volume * (0.5 + abs(pattern) * 0.5);

    var color = hsv_to_rgb(vec3<f32>(fract(depth_hue), saturation, brightness));

    // Blend with frequency zone colors
    let zone_color = bass_color + mid_color + treble_color;
    let zone_blend = uniforms.spectral_flux * 0.5;
    color = mix(color, zone_color * brightness, zone_blend);

    // Beat-driven color flashes
    let beat_flash = 1.0 + uniforms.beat_strength * sin(uniforms.time * 12.0) * 0.4;
    color = color * beat_flash;

    // Downbeat creates tunnel-wide color shifts
    let downbeat_shift = uniforms.downbeat_detected * sin(angle * 2.0 + depth) * 0.3;
    color.r += downbeat_shift;
    color.b += -downbeat_shift;

    return color;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize coordinates to screen center
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    let uv = (in.tex_coords * 2.0 - 1.0) * vec2<f32>(resolution.x / resolution.y, 1.0);

    // Transform to tunnel coordinates
    let tunnel_coord = tunnel_coordinates(uv);

    // Generate tunnel pattern
    let pattern = generate_tunnel_pattern(tunnel_coord);

    // Generate audio-reactive color
    var color = get_tunnel_color(pattern, tunnel_coord);

    // Edge vignette for tunnel depth perception
    let edge_distance = length(uv);
    let vignette = 1.0 - smoothstep(0.8, 1.4, edge_distance);
    color = color * vignette;

    // Zero crossing rate affects tunnel "roughness"
    let roughness = 1.0 + uniforms.zero_crossing_rate * 0.2;
    color = pow(color, vec3<f32>(1.0 / roughness));

    // Apply global intensity
    color = color * uniforms.color_intensity;

    // Ensure color values stay in valid range
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}