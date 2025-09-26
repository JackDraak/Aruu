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

// Simulate frequency spectrum display
fn get_frequency_bar_height(freq_position: f32) -> f32 {
    // Determine which frequency band we're in
    let band_position = freq_position * 5.0;
    let band_index = clamp(floor(band_position), 0.0, 4.0);
    let band_fraction = fract(band_position);

    // Get current and next band values using conditional logic instead of array indexing
    var current_band: f32;
    var next_band: f32;

    if (band_index < 0.5) {
        // Band 0: sub_bass
        current_band = uniforms.sub_bass;
        next_band = uniforms.bass;
    } else if (band_index < 1.5) {
        // Band 1: bass
        current_band = uniforms.bass;
        next_band = uniforms.mid;
    } else if (band_index < 2.5) {
        // Band 2: mid
        current_band = uniforms.mid;
        next_band = uniforms.treble;
    } else if (band_index < 3.5) {
        // Band 3: treble
        current_band = uniforms.treble;
        next_band = uniforms.presence;
    } else {
        // Band 4: presence
        current_band = uniforms.presence;
        next_band = uniforms.presence; // No next band, use same
    }

    // Interpolate between bands for smooth transitions
    let interpolated_height = mix(current_band, next_band, band_fraction);

    // Beat-driven amplitude modulation
    let beat_boost = 1.0 + uniforms.beat_strength * sin(uniforms.time * 8.0 + freq_position * 10.0) * 0.3;
    let final_height = interpolated_height * beat_boost;

    // Onset creates sudden spikes across all frequencies
    let onset_spike = uniforms.onset_strength * exp(-abs(freq_position - 0.5) * 4.0) * 0.4;

    return final_height + onset_spike;
}

// Generate spectrum analyzer bars
fn generate_spectrum_bars(uv: vec2<f32>) -> f32 {
    // Horizontal position maps to frequency (0 = low, 1 = high)
    let freq_pos = (uv.x + 1.0) * 0.5; // Convert from [-1,1] to [0,1]

    // Vertical position maps to amplitude (0 = bottom, 1 = top)
    let amplitude_pos = (uv.y + 1.0) * 0.5;

    // Get the target height for this frequency
    let target_height = get_frequency_bar_height(freq_pos);

    // Create bar visualization with smooth edges
    let bar_width = 0.15; // Width of each frequency bar
    let bar_spacing = 0.2; // Spacing between bars

    // Calculate which bar we're in
    let bar_count = 20.0;
    let bar_x = fract(freq_pos * bar_count);
    let bar_center = 0.5;

    // Distance from bar center
    let bar_distance = abs(bar_x - bar_center);

    // Create bar shape with soft edges
    let bar_mask = 1.0 - smoothstep(bar_width * 0.4, bar_width * 0.6, bar_distance);

    // Height mask - create the bar visualization
    let height_mask = 1.0 - smoothstep(target_height * 0.9, target_height, amplitude_pos);

    // Combine masks
    let bar_intensity = bar_mask * height_mask;

    // Add frequency band reflections for visual richness
    let reflection_height = target_height * 0.3;
    let reflection_mask = smoothstep(-0.1, reflection_height, -amplitude_pos) * bar_mask * 0.4;

    return bar_intensity + reflection_mask;
}

// Generate waveform display
fn generate_waveform(uv: vec2<f32>) -> f32 {
    let time = uniforms.time;

    // BPM-synchronized waveform scrolling
    let bpm_speed = uniforms.estimated_bpm / 120.0;
    let scroll_speed = 2.0 + bpm_speed * 3.0;
    let scrolled_x = uv.x + time * scroll_speed;

    // Generate synthetic waveform based on frequency content
    let wave_bass = sin(scrolled_x * 8.0) * uniforms.bass * 0.4;
    let wave_mid = sin(scrolled_x * 24.0) * uniforms.mid * 0.3;
    let wave_treble = sin(scrolled_x * 48.0) * uniforms.treble * 0.2;
    let wave_noise = sin(scrolled_x * 120.0 + uniforms.spectral_flux * 50.0) * uniforms.presence * 0.1;

    // Combine waveform components
    let combined_wave = (wave_bass + wave_mid + wave_treble + wave_noise) * uniforms.overall_volume;

    // Beat-driven amplitude modulation
    let beat_modulation = 1.0 + uniforms.beat_strength * sin(time * 6.0) * 0.5;
    let final_wave = combined_wave * beat_modulation;

    // Create waveform line with thickness
    let line_thickness = 0.02 + uniforms.dynamic_range * 0.02;
    let waveform_intensity = 1.0 - smoothstep(0.0, line_thickness, abs(uv.y - final_wave));

    // Add waveform glow effect
    let glow_thickness = line_thickness * 3.0;
    let glow_intensity = exp(-abs(uv.y - final_wave) / glow_thickness) * 0.3;

    return waveform_intensity + glow_intensity;
}

// Generate circular spectrum (radial frequency display)
fn generate_circular_spectrum(uv: vec2<f32>) -> f32 {
    let radius = length(uv);
    let angle = atan2(uv.y, uv.x) + 3.14159; // Convert to 0-2π range

    // Map angle to frequency position
    let freq_pos = angle / (2.0 * 3.14159);
    let target_radius = 0.3 + get_frequency_bar_height(freq_pos) * 0.4;

    // Create circular bars
    let ring_thickness = 0.05;
    var ring_intensity = 1.0 - smoothstep(target_radius - ring_thickness, target_radius, radius);
    ring_intensity *= smoothstep(target_radius - ring_thickness * 3.0, target_radius - ring_thickness, radius);

    // Add radial segments for visual separation
    let segment_count = 32.0;
    let segment_angle = fract(angle / (2.0 * 3.14159) * segment_count);
    let segment_mask = smoothstep(0.0, 0.1, segment_angle) * smoothstep(1.0, 0.9, segment_angle);

    return ring_intensity * segment_mask;
}

// Generate particle system representing spectral energy
fn generate_spectral_particles(uv: vec2<f32>) -> f32 {
    let time = uniforms.time;

    // Create particle grid based on spectral characteristics
    let grid_density = 8.0 + uniforms.spectral_flux * 8.0;
    let grid_pos = floor(uv * grid_density);
    let cell_pos = fract(uv * grid_density) - 0.5;

    // Particle ID for consistent randomization
    let particle_id = grid_pos.x * 127.0 + grid_pos.y * 311.0;
    let random_seed = fract(sin(particle_id) * 43758.5453);

    // Map particle position to frequency band
    let freq_pos = (grid_pos.x / grid_density + 1.0) * 0.5;
    let particle_energy = get_frequency_bar_height(freq_pos);

    // Particle animation
    let particle_phase = time * 2.0 + random_seed * 6.28318;
    let particle_offset = vec2<f32>(
        sin(particle_phase) * particle_energy * 0.1,
        cos(particle_phase * 1.3) * particle_energy * 0.15
    );

    // Distance to animated particle
    let particle_distance = length(cell_pos - particle_offset);

    // Particle size based on energy and zero crossing rate
    let particle_size = 0.1 + particle_energy * 0.2 + uniforms.zero_crossing_rate * 0.05;

    // Particle brightness
    let particle_brightness = exp(-particle_distance / particle_size) * particle_energy;

    return particle_brightness;
}

// Combine all spectralizer elements
fn generate_spectralizer_pattern(uv: vec2<f32>) -> f32 {
    // Different visualization modes based on audio characteristics
    let spectrum_bars = generate_spectrum_bars(uv);
    let waveform = generate_waveform(uv * vec2<f32>(2.0, 1.0)); // Stretch waveform horizontally
    let circular_spectrum = generate_circular_spectrum(uv);
    let spectral_particles = generate_spectral_particles(uv);

    // Blend visualization modes based on audio content
    let bar_weight = uniforms.bass + uniforms.mid; // Traditional spectrum for rhythmic content
    let wave_weight = uniforms.treble + uniforms.presence; // Waveform for melodic content
    let circular_weight = uniforms.pitch_confidence; // Circular for harmonic content
    let particle_weight = uniforms.spectral_flux; // Particles for dynamic content

    // Normalize weights
    let total_weight = bar_weight + wave_weight + circular_weight + particle_weight + 0.1;

    let final_pattern =
        (spectrum_bars * bar_weight +
         waveform * wave_weight +
         circular_spectrum * circular_weight +
         spectral_particles * particle_weight) / total_weight;

    return final_pattern;
}

// Audio-reactive color generation for spectralizer
fn get_spectralizer_color(pattern: f32, uv: vec2<f32>) -> vec3<f32> {
    // Map horizontal position to frequency-based hues
    let freq_pos = (uv.x + 1.0) * 0.5;

    // Frequency to hue mapping (low = red/orange, high = blue/purple)
    let base_hue = 0.0 + freq_pos * 0.7; // Red to blue spectrum

    // Spectral characteristics modulate color
    let spectral_hue = base_hue + uniforms.spectral_centroid * 0.0001 + uniforms.time * 0.02;

    // Pattern intensity affects color brightness
    let saturation = uniforms.saturation * (0.8 + uniforms.pitch_confidence * 0.2);
    let brightness = pattern * uniforms.overall_volume * (0.7 + uniforms.tempo_confidence * 0.3);

    // Generate base color
    var color = hsv_to_rgb(vec3<f32>(fract(spectral_hue), saturation, brightness));

    // Frequency band color influences
    let low_freq_influence = (uniforms.sub_bass + uniforms.bass) * vec3<f32>(1.0, 0.3, 0.1);
    let mid_freq_influence = uniforms.mid * vec3<f32>(0.3, 1.0, 0.3);
    let high_freq_influence = (uniforms.treble + uniforms.presence) * vec3<f32>(0.1, 0.4, 1.0);

    let freq_color = low_freq_influence + mid_freq_influence + high_freq_influence;
    let freq_blend = uniforms.spectral_flux * 0.4;

    color = mix(color, freq_color * brightness, freq_blend);

    // Beat-driven color pulsing
    let beat_pulse = 1.0 + uniforms.beat_strength * sin(uniforms.time * 8.0 + freq_pos * 20.0) * 0.3;
    color = color * beat_pulse;

    // Onset creates bright flashes
    let onset_flash = uniforms.onset_strength * pattern * 0.5;
    color += vec3<f32>(onset_flash);

    return color;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize coordinates to screen center
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    let uv = (in.tex_coords * 2.0 - 1.0) * vec2<f32>(resolution.x / resolution.y, 1.0);

    // Generate spectralizer pattern
    let pattern = generate_spectralizer_pattern(uv);

    // Generate audio-reactive color
    var color = get_spectralizer_color(pattern, uv);

    // Background gradient for better contrast
    let background_gradient = max(0.0, 1.0 - length(uv) * 0.5) * 0.1;
    let background_color = vec3<f32>(0.05, 0.02, 0.1) * background_gradient;

    color += background_color;

    // Apply global intensity
    color = color * uniforms.color_intensity;

    // Dynamic range affects overall brightness
    let brightness_factor = 0.8 + uniforms.dynamic_range * 0.4;
    color = color * brightness_factor;

    // Ensure color values stay in valid range
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}