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

// Enhanced noise functions for particle randomization
fn hash21(p: vec2<f32>) -> f32 {
    var p_int = vec2<i32>(floor(p));
    let p_fract = fract(p);

    p_int = (p_int * vec2<i32>(1597, 2137)) % vec2<i32>(289);
    let n = p_int.x * p_int.y * (p_int.x + p_int.y);

    return fract(sin(f32(n)) * 43758.5453);
}

fn hash22(p: vec2<f32>) -> vec2<f32> {
    let q = vec2<f32>(dot(p, vec2<f32>(127.1, 311.7)),
                      dot(p, vec2<f32>(269.5, 183.3)));
    return -1.0 + 2.0 * fract(sin(q) * 43758.5453123);
}

// Simulate individual particle behavior
fn simulate_particle(particle_id: vec2<f32>, time: f32) -> vec4<f32> {
    let random_seed = hash21(particle_id);
    let velocity_seed = hash22(particle_id * 1.3);

    // BPM-synchronized particle lifecycle
    let bpm_speed = uniforms.estimated_bpm / 120.0;
    let lifecycle_speed = 0.8 + bpm_speed * 1.5;
    let lifecycle = fract(time * lifecycle_speed + random_seed);

    // Particle birth and death tied to onsets
    let onset_birth_probability = uniforms.onset_strength * 0.7;
    let birth_threshold = 0.1 + onset_birth_probability;
    let alive = step(birth_threshold, lifecycle) * (1.0 - step(0.9, lifecycle));

    // Initial position (birth location)
    let birth_position = (hash22(particle_id * 2.1) - 0.5) * 2.0;

    // Velocity influenced by frequency bands
    let bass_velocity = velocity_seed * uniforms.bass * 2.0;
    let treble_scatter = hash22(particle_id * 3.7) * uniforms.treble * uniforms.presence * 1.5;
    let total_velocity = bass_velocity + treble_scatter;

    // Beat-driven explosive motion
    let beat_boost = uniforms.beat_strength * sin(time * 8.0 + random_seed * 6.28318) * 1.5;
    let explosive_velocity = total_velocity * (1.0 + beat_boost);

    // Current position
    let age = (lifecycle - birth_threshold) / (0.9 - birth_threshold);
    let position = birth_position + explosive_velocity * age;

    // Size influenced by dynamic range
    let base_size = 0.02 + uniforms.dynamic_range * 0.05;
    let age_size = base_size * (1.0 - age * 0.7); // Shrink with age
    let beat_size = age_size * (1.0 + uniforms.beat_strength * 0.5);

    return vec4<f32>(position, beat_size, alive);
}

// Generate particle field
fn generate_particle_field(uv: vec2<f32>) -> f32 {
    let time = uniforms.time;

    // Grid-based particle spawning
    let grid_scale = 8.0 + uniforms.spectral_flux * 12.0;
    let grid = floor(uv * grid_scale);
    let cell = fract(uv * grid_scale) - 0.5;

    var total_brightness = 0.0;

    // Sample multiple particles per grid cell for density
    let samples_per_cell = 3;
    for (var i = 0; i < samples_per_cell; i = i + 1) {
        let particle_offset = vec2<f32>(f32(i), f32(i) * 1.618) * 0.3;
        let particle_id = grid + particle_offset;

        let particle = simulate_particle(particle_id, time);
        let particle_pos = particle.xy;
        let particle_size = particle.z;
        let particle_alive = particle.w;

        // Distance from current pixel to particle
        let particle_distance = length(cell - particle_pos);

        // Particle brightness with soft falloff
        let brightness = particle_alive * exp(-particle_distance / max(particle_size, 0.01));

        // Frequency-based particle intensity modulation
        let freq_modulation = 0.5 + uniforms.mid * 0.3 + uniforms.treble * 0.4;
        total_brightness += brightness * freq_modulation;
    }

    // Spectral flux creates particle density bursts
    let flux_burst = uniforms.spectral_flux * sin(length(uv) * 10.0 + time * 5.0) * 0.3;
    total_brightness += flux_burst;

    // Zero crossing rate affects particle sharpness
    let sharpness = 1.0 + uniforms.zero_crossing_rate * 2.0;
    total_brightness = pow(total_brightness, sharpness);

    return total_brightness;
}

// Audio-reactive color generation for particles
fn get_particle_color(brightness: f32, uv: vec2<f32>) -> vec3<f32> {
    // Base hue influenced by spectral characteristics
    let spectral_hue = uniforms.spectral_centroid * 0.0001 + uniforms.time * 0.08;

    // Particle-specific hue variation
    let position_hash = hash21(floor(uv * 10.0));
    let particle_hue = spectral_hue + position_hash * 0.3 + brightness * 0.2;

    // Frequency band color influences
    let bass_color = vec3<f32>(1.0, 0.4, 0.2) * uniforms.bass;
    let mid_color = vec3<f32>(0.3, 1.0, 0.5) * uniforms.mid;
    let treble_color = vec3<f32>(0.2, 0.5, 1.0) * uniforms.treble;
    let presence_color = vec3<f32>(1.0, 0.8, 1.0) * uniforms.presence;

    // Blend frequency colors
    let freq_color = bass_color + mid_color + treble_color + presence_color;
    let normalized_freq_color = normalize(freq_color + vec3<f32>(0.1));

    // HSV color generation
    let saturation = uniforms.saturation * (0.7 + uniforms.pitch_confidence * 0.3);
    let hsv_brightness = uniforms.overall_volume * brightness;

    var color = hsv_to_rgb(vec3<f32>(fract(particle_hue), saturation, hsv_brightness));

    // Blend with frequency-based color
    let freq_blend = uniforms.spectral_flux * 0.6;
    color = mix(color, normalized_freq_color * hsv_brightness, freq_blend);

    // Beat-driven color flashing
    let beat_flash = 1.0 + uniforms.beat_strength * sin(uniforms.time * 10.0 + position_hash * 6.28318) * 0.4;
    color = color * beat_flash;

    // Onset creates white-hot particle cores
    let onset_core = uniforms.onset_strength * brightness * 0.8;
    color += vec3<f32>(onset_core);

    return color;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize coordinates to screen center
    let resolution = vec2<f32>(1200.0, 800.0); // TODO: Make this dynamic
    let uv = (in.tex_coords * 2.0 - 1.0) * vec2<f32>(resolution.x / resolution.y, 1.0);

    // Generate particle field brightness
    let brightness = generate_particle_field(uv);

    // Generate audio-reactive color
    var color = get_particle_color(brightness, uv);

    // Background gradient influenced by sub-bass
    let radius = length(uv);
    let background_intensity = uniforms.sub_bass * exp(-radius * 2.0) * 0.2;
    let background_color = vec3<f32>(0.1, 0.05, 0.2) * background_intensity;

    color += background_color;

    // Apply global intensity
    color = color * uniforms.color_intensity;

    // Dynamic range affects overall contrast
    let contrast = 1.0 + uniforms.dynamic_range * 0.3;
    color = pow(color, vec3<f32>(1.0 / contrast));

    // Ensure color values stay in valid range
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}