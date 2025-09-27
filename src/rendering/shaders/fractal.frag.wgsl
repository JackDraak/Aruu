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

// Complex number operations for fractal mathematics
fn complex_mult(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn complex_square(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

fn complex_magnitude_sq(z: vec2<f32>) -> f32 {
    return z.x * z.x + z.y * z.y;
}

// Audio-reactive Mandelbrot fractal
fn mandelbrot_fractal(uv: vec2<f32>) -> f32 {
    // Audio-reactive zoom and translation
    let bass_zoom = 0.5 + uniforms.bass * 2.0 + uniforms.sub_bass * 1.5;
    let safe_beat_offset = uniforms.beat_strength * uniforms.safety_beat_intensity;
    let beat_offset = safe_beat_offset * sin(uniforms.time * 2.0) * 0.05; // Reduced speed and intensity

    // Spectral centroid influences fractal center point
    let spectral_offset = vec2<f32>(
        uniforms.spectral_centroid * 0.00001 - 0.5,
        uniforms.spectral_rolloff * 0.00001 - 0.3
    );

    // Scale and translate coordinates
    let c = (uv - spectral_offset) / bass_zoom + vec2<f32>(beat_offset, 0.0);

    var z = vec2<f32>(0.0, 0.0);
    let max_iterations = 64;
    var iteration = 0.0;

    // Dynamic iteration count based on dynamic range
    let dynamic_max_iter = f32(max_iterations) * (0.5 + uniforms.dynamic_range * 0.5);

    for (var i = 0; i < max_iterations; i = i + 1) {
        if (complex_magnitude_sq(z) > 4.0 || f32(i) > dynamic_max_iter) {
            break;
        }

        // Audio-reactive fractal equation variations
        let safe_onset_variation = uniforms.onset_strength * uniforms.safety_onset_intensity;
        let onset_variation = safe_onset_variation * sin(uniforms.time * 4.0) * 0.05; // Reduced speed and intensity
        let modified_c = c + vec2<f32>(onset_variation, 0.0);

        z = complex_square(z) + modified_c;

        // Pitch confidence affects iteration evolution
        if (uniforms.pitch_confidence > 0.7) {
            let harmonic_factor = sin(f32(i) * 0.5) * 0.05;
            z += vec2<f32>(harmonic_factor);
        }

        iteration += 1.0;
    }

    // Smooth iteration count for better gradients
    var smooth_iter = iteration;
    if (complex_magnitude_sq(z) <= 4.0) {
        smooth_iter = dynamic_max_iter;
    } else {
        let log_zn = log(complex_magnitude_sq(z)) * 0.5;
        smooth_iter = iteration + 1.0 - log(log_zn) / log(2.0);
    }

    return smooth_iter / dynamic_max_iter;
}

// Julia set variation driven by audio
fn julia_fractal(uv: vec2<f32>) -> f32 {
    // BPM-synchronized parameter evolution
    let bpm_speed = uniforms.estimated_bpm / 120.0;
    let evolution_speed = 0.3 + bpm_speed * 0.5;
    let param_time = uniforms.time * evolution_speed;

    // Audio-reactive Julia set parameters
    let bass_influence = uniforms.bass * 0.5;
    let treble_influence = uniforms.treble * 0.3;

    let julia_c = vec2<f32>(
        cos(param_time) * (0.3 + bass_influence) + uniforms.spectral_flux * 0.2,
        sin(param_time * 1.3) * (0.2 + treble_influence) + uniforms.zero_crossing_rate * 0.1
    );

    // Scale coordinates
    let zoom = 1.5 + uniforms.mid * 1.0;
    var z = uv / zoom;

    let max_iterations = 48;
    var iteration = 0.0;

    for (var i = 0; i < max_iterations; i = i + 1) {
        if (complex_magnitude_sq(z) > 4.0) {
            break;
        }

        z = complex_square(z) + julia_c;

        // Beat-driven iteration modifications
        let safe_beat_threshold = uniforms.beat_strength * uniforms.safety_beat_intensity;
        if (safe_beat_threshold > 0.3) { // Reduced threshold
            z += vec2<f32>(sin(f32(i) * 0.8) * 0.02);
        }

        iteration += 1.0;
    }

    return iteration / f32(max_iterations);
}

// Burning ship fractal for variety
fn burning_ship_fractal(uv: vec2<f32>) -> f32 {
    let zoom = 0.8 + uniforms.presence * 1.5;
    let c = (uv + vec2<f32>(-1.8, -0.1)) / zoom;

    var z = vec2<f32>(0.0, 0.0);
    let max_iterations = 32;
    var iteration = 0.0;

    for (var i = 0; i < max_iterations; i = i + 1) {
        if (complex_magnitude_sq(z) > 4.0) {
            break;
        }

        // Burning ship modification: take absolute values
        z = vec2<f32>(abs(z.x), abs(z.y));
        z = complex_square(z) + c;

        iteration += 1.0;
    }

    return iteration / f32(max_iterations);
}

// Generate combined fractal pattern
fn generate_fractal_pattern(uv: vec2<f32>) -> f32 {
    // Frequency bands determine fractal type blending
    let mandelbrot_weight = uniforms.bass + uniforms.sub_bass;
    let julia_weight = uniforms.mid + uniforms.treble;
    let burning_ship_weight = uniforms.presence;

    // Normalize weights
    let total_weight = mandelbrot_weight + julia_weight + burning_ship_weight + 0.1;
    let norm_mandelbrot = mandelbrot_weight / total_weight;
    let norm_julia = julia_weight / total_weight;
    let norm_burning = burning_ship_weight / total_weight;

    // Calculate fractals
    let mandelbrot = mandelbrot_fractal(uv);
    let julia = julia_fractal(uv);
    let burning_ship = burning_ship_fractal(uv);

    // Blend fractals based on audio content
    var pattern = mandelbrot * norm_mandelbrot + julia * norm_julia + burning_ship * norm_burning;

    // Spectral flux creates pattern modulation
    let flux_modulation = 1.0 + uniforms.spectral_flux * sin(pattern * 20.0) * 0.3;
    pattern = pattern * flux_modulation;

    return pattern;
}

// Audio-reactive fractal coloring
fn get_fractal_color(pattern: f32, uv: vec2<f32>) -> vec3<f32> {
    // Base hue evolution with time and spectral characteristics
    let base_hue = uniforms.time * 0.05 + uniforms.spectral_centroid * 0.0001;

    // Pattern-based hue shifts
    let pattern_hue = base_hue + pattern * 2.0 + length(uv) * 0.2;

    // Frequency band color influences
    let bass_hue_shift = uniforms.bass * 0.1; // Warm shift
    let treble_hue_shift = uniforms.treble * -0.15; // Cool shift
    let mid_saturation = uniforms.mid * 0.4;

    let final_hue = pattern_hue + bass_hue_shift + treble_hue_shift;

    // Dynamic saturation based on audio characteristics
    let base_saturation = uniforms.saturation * (0.6 + uniforms.pitch_confidence * 0.4);
    let pattern_saturation = base_saturation * (0.8 + mid_saturation);

    // Brightness influenced by pattern and volume
    let base_brightness = uniforms.overall_volume * (0.3 + pattern * 0.7);

    // Safe beat-driven brightness pulses
    let safe_beat_brightness = uniforms.beat_strength * uniforms.safety_beat_intensity;
    let beat_brightness = 1.0 + safe_beat_brightness * sin(uniforms.time * 3.0 + pattern * 5.0) * 0.2; // Reduced speed and intensity
    let final_brightness = base_brightness * beat_brightness;

    // Generate HSV color
    var color = hsv_to_rgb(vec3<f32>(fract(final_hue), pattern_saturation, final_brightness));

    // Onset creates color highlights
    let safe_onset_highlight = uniforms.onset_strength * uniforms.safety_onset_intensity;
    let onset_highlight = safe_onset_highlight * (1.0 - pattern) * 0.25; // Reduced intensity
    color += vec3<f32>(onset_highlight, onset_highlight * 0.7, onset_highlight * 0.4);

    // Zero crossing rate affects color complexity
    let complexity_factor = 1.0 + uniforms.zero_crossing_rate * 0.3;
    color = pow(color, vec3<f32>(1.0 / complexity_factor));

    return color;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize coordinates to screen center
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    let uv = (in.tex_coords * 2.0 - 1.0) * vec2<f32>(resolution.x / resolution.y, 1.0);

    // Generate fractal pattern
    let pattern = generate_fractal_pattern(uv);

    // Generate audio-reactive color
    var color = get_fractal_color(pattern, uv);

    // Edge enhancement for fractal boundaries
    let edge_threshold = 0.1;
    let edge_intensity = abs(fract(pattern * 10.0) - 0.5) * 2.0;
    let edge_enhancement = smoothstep(edge_threshold, 1.0, edge_intensity) * uniforms.treble * 0.3;

    color += vec3<f32>(edge_enhancement);

    // Apply safe global intensity
    color = color * uniforms.color_intensity * uniforms.safety_brightness_range;

    // Safe dynamic range with pattern complexity control
    let safe_dynamic_factor = uniforms.dynamic_range * uniforms.safety_pattern_complexity;
    let contrast = 1.0 + safe_dynamic_factor * 0.2; // Reduced from 0.4
    color = pow(color, vec3<f32>(1.0 / contrast));

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