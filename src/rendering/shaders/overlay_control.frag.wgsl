// Control panel overlay fragment shader - renders UI controls in top-left

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

// Enhanced SDF functions for professional UI elements
fn sdf_box(pos: vec2<f32>, size: vec2<f32>) -> f32 {
    let d = abs(pos) - size;
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
}

fn sdf_rounded_box(pos: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(pos) - size + radius;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - radius;
}

fn sdf_circle(pos: vec2<f32>, radius: f32) -> f32 {
    return length(pos) - radius;
}

fn draw_icon_pattern(pos: vec2<f32>, icon_type: i32) -> f32 {
    // Simple geometric patterns for icons
    let centered_pos = pos - vec2<f32>(0.5, 0.5);

    switch icon_type {
        case 0: { // Previous arrow
            if (abs(centered_pos.x + 0.1) < 0.05 && abs(centered_pos.y) < 0.15) {
                return 1.0;
            }
            if (centered_pos.x > -0.1 && centered_pos.x < 0.1 &&
                abs(centered_pos.y) < (0.15 - abs(centered_pos.x + 0.1) * 1.5)) {
                return 1.0;
            }
        }
        case 1: { // Next arrow
            if (abs(centered_pos.x - 0.1) < 0.05 && abs(centered_pos.y) < 0.15) {
                return 1.0;
            }
            if (centered_pos.x > -0.1 && centered_pos.x < 0.1 &&
                abs(centered_pos.y) < (0.15 - abs(centered_pos.x - 0.1) * 1.5)) {
                return 1.0;
            }
        }
        case 2: { // Open folder
            if (abs(centered_pos.y + 0.05) < 0.02 && abs(centered_pos.x) < 0.12) {
                return 1.0;
            }
            if (abs(centered_pos.y) < 0.08 && abs(centered_pos.x) < 0.1) {
                return 0.3;
            }
        }
        case 3: { // Emergency stop
            if (length(centered_pos) < 0.12) {
                return 1.0;
            }
        }
        case 4: { // Play/Pause
            if (uniforms.ui_is_playing > 0.5) {
                // Pause icon (two bars)
                if (abs(centered_pos.x - 0.04) < 0.025 || abs(centered_pos.x + 0.04) < 0.025) {
                    if (abs(centered_pos.y) < 0.1) {
                        return 1.0;
                    }
                }
            } else {
                // Play icon (triangle)
                if (centered_pos.x > -0.06 && centered_pos.x < 0.08 &&
                    abs(centered_pos.y) < (0.1 - (centered_pos.x + 0.06) * 0.7)) {
                    return 1.0;
                }
            }
        }
        default: {
            return 0.0;
        }
    }
    return 0.0;
}

fn draw_text_pattern(pos: vec2<f32>, scale: f32) -> f32 {
    // Enhanced geometric text simulation
    let grid_x = floor(pos.x / scale) * scale;
    let grid_y = floor(pos.y / scale) * scale;
    let cell_x = (pos.x - grid_x) / scale;
    let cell_y = (pos.y - grid_y) / scale;

    // Create readable text-like patterns
    if (cell_x > 0.05 && cell_x < 0.95 && cell_y > 0.15 && cell_y < 0.85) {
        let pattern = sin(pos.x * 25.0) * sin(pos.y * 15.0);
        if (pattern > 0.2) {
            return 0.8 + pattern * 0.2;
        }
    }
    return 0.0;
}

fn is_mouse_over_area(area_center: vec2<f32>, area_size: vec2<f32>) -> bool {
    let mouse_local = vec2<f32>(uniforms.mouse_x, uniforms.mouse_y);
    let area_min = area_center - area_size * 0.5;
    let area_max = area_center + area_size * 0.5;

    return mouse_local.x >= area_min.x && mouse_local.x <= area_max.x &&
           mouse_local.y >= area_min.y && mouse_local.y <= area_max.y;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let screen_pos = input.screen_pos;

    // Only render in top-left corner (x < 0.4, y < 0.3)
    if (screen_pos.x > 0.4 || screen_pos.y > 0.3) {
        discard;
    }

    // Local coordinates within the control panel (0.0 to 1.0)
    let local_x = screen_pos.x / 0.4;
    let local_y = screen_pos.y / 0.3;

    // Semi-transparent dark background with subtle border
    var color = vec4<f32>(0.06, 0.06, 0.13, 0.9);

    // Panel border with constructivist styling
    let panel_center = vec2<f32>(local_x - 0.5, local_y - 0.5);
    let border_sdf = sdf_rounded_box(panel_center, vec2<f32>(0.48, 0.48), 0.03);

    if (border_sdf > -0.008 && border_sdf < 0.0) {
        color = vec4<f32>(0.25, 0.3, 0.4, 0.95); // Subtle blue-gray border
    }

    // Header section (0.0 - 0.18)
    if (local_y < 0.18) {
        color = vec4<f32>(0.12, 0.15, 0.22, 0.95);

        // Header title
        if (local_y > 0.05 && local_y < 0.13 && local_x > 0.05 && local_x < 0.95) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 8.0, (local_y - 0.05) * 20.0), 0.12);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.8, 0.85, 0.9, 0.95);
            }
        }

        // Header underline
        if (abs(local_y - 0.15) < 0.002) {
            color = vec4<f32>(0.4, 0.5, 0.6, 0.9);
        }
    }

    // Volume control section (0.18 - 0.38)
    if (local_y >= 0.18 && local_y < 0.38) {
        // Volume label
        if (local_y > 0.20 && local_y < 0.26 && local_x > 0.05 && local_x < 0.25) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 15.0, (local_y - 0.20) * 25.0), 0.08);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.7, 0.75, 0.8, 0.9);
            }
        }

        // Volume slider track
        let slider_y = 0.31;
        let slider_height = 0.04;
        if (local_y >= slider_y - slider_height * 0.5 && local_y < slider_y + slider_height * 0.5 &&
            local_x >= 0.1 && local_x < 0.9) {

            // Track background
            color = vec4<f32>(0.2, 0.25, 0.3, 0.9);

            // Volume level fill with audio-reactive glow
            let volume_width = uniforms.ui_volume * 0.8;
            if (local_x < 0.1 + volume_width) {
                let audio_pulse = uniforms.overall_volume * 0.3;
                color = vec4<f32>(0.3 + audio_pulse, 0.7 + audio_pulse * 0.2, 0.4, 0.95);
            }

            // Volume handle with hover effect
            let handle_x = 0.1 + volume_width;
            let handle_distance = abs(local_x - handle_x);
            let handle_mouse_over = is_mouse_over_area(vec2<f32>(handle_x * 0.4, slider_y * 0.3), vec2<f32>(0.04, 0.03));

            if (handle_distance < 0.025) {
                if (handle_mouse_over) {
                    color = vec4<f32>(0.9, 0.95, 1.0, 1.0);
                } else {
                    color = vec4<f32>(0.75, 0.8, 0.85, 0.95);
                }
            }
        }
    }

    // File control section (0.38 - 0.62)
    if (local_y >= 0.38 && local_y < 0.62) {
        // Section label
        if (local_y > 0.40 && local_y < 0.46 && local_x > 0.05 && local_x < 0.5) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 12.0, (local_y - 0.40) * 25.0), 0.1);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.7, 0.75, 0.8, 0.9);
            }
        }

        let button_y = 0.52;
        let button_radius = 0.045;

        // Previous button
        let prev_center = vec2<f32>(0.2, button_y);
        let prev_distance = distance(vec2<f32>(local_x, local_y), prev_center);
        if (prev_distance < button_radius) {
            let prev_mouse_over = is_mouse_over_area(vec2<f32>(prev_center.x * 0.4, prev_center.y * 0.3), vec2<f32>(0.08, 0.08));

            if (prev_mouse_over && uniforms.mouse_pressed > 0.5) {
                color = vec4<f32>(0.5, 0.6, 0.7, 0.95); // Pressed state
            } else if (prev_mouse_over) {
                color = vec4<f32>(0.4, 0.5, 0.6, 0.95); // Hover state
            } else {
                color = vec4<f32>(0.25, 0.35, 0.45, 0.9); // Normal state
            }

            // Previous arrow icon
            let icon_pos = (vec2<f32>(local_x, local_y) - prev_center) / (button_radius * 1.8) + vec2<f32>(0.5, 0.5);
            let icon_intensity = draw_icon_pattern(icon_pos, 0);
            if (icon_intensity > 0.5) {
                color = mix(color, vec4<f32>(0.9, 0.95, 1.0, 1.0), icon_intensity);
            }
        }

        // Open file button (rectangular)
        let open_center = vec2<f32>(0.5, button_y);
        let open_size = vec2<f32>(0.08, 0.05);
        let open_sdf = sdf_rounded_box(vec2<f32>(local_x, local_y) - open_center, open_size, 0.01);
        if (open_sdf < 0.0) {
            let open_mouse_over = is_mouse_over_area(vec2<f32>(open_center.x * 0.4, open_center.y * 0.3), vec2<f32>(0.12, 0.08));

            if (open_mouse_over && uniforms.mouse_pressed > 0.5) {
                color = vec4<f32>(0.4, 0.7, 0.5, 0.95); // Pressed state
            } else if (open_mouse_over) {
                color = vec4<f32>(0.35, 0.65, 0.45, 0.95); // Hover state
            } else {
                color = vec4<f32>(0.25, 0.5, 0.35, 0.9); // Normal state
            }

            // Open folder icon
            let icon_pos = (vec2<f32>(local_x, local_y) - open_center) / (open_size.x * 2.2) + vec2<f32>(0.5, 0.5);
            let icon_intensity = draw_icon_pattern(icon_pos, 2);
            if (icon_intensity > 0.3) {
                color = mix(color, vec4<f32>(0.9, 0.95, 1.0, 1.0), icon_intensity);
            }
        }

        // Next button
        let next_center = vec2<f32>(0.8, button_y);
        let next_distance = distance(vec2<f32>(local_x, local_y), next_center);
        if (next_distance < button_radius) {
            let next_mouse_over = is_mouse_over_area(vec2<f32>(next_center.x * 0.4, next_center.y * 0.3), vec2<f32>(0.08, 0.08));

            if (next_mouse_over && uniforms.mouse_pressed > 0.5) {
                color = vec4<f32>(0.5, 0.6, 0.7, 0.95); // Pressed state
            } else if (next_mouse_over) {
                color = vec4<f32>(0.4, 0.5, 0.6, 0.95); // Hover state
            } else {
                color = vec4<f32>(0.25, 0.35, 0.45, 0.9); // Normal state
            }

            // Next arrow icon
            let icon_pos = (vec2<f32>(local_x, local_y) - next_center) / (button_radius * 1.8) + vec2<f32>(0.5, 0.5);
            let icon_intensity = draw_icon_pattern(icon_pos, 1);
            if (icon_intensity > 0.5) {
                color = mix(color, vec4<f32>(0.9, 0.95, 1.0, 1.0), icon_intensity);
            }
        }
    }

    // Safety and emergency control section (0.62 - 0.82)
    if (local_y >= 0.62 && local_y < 0.82) {
        // Emergency stop button (large, prominent)
        let emergency_center = vec2<f32>(0.5, 0.72);
        let emergency_radius = 0.08;
        let emergency_distance = distance(vec2<f32>(local_x, local_y), emergency_center);

        if (emergency_distance < emergency_radius) {
            let emergency_mouse_over = is_mouse_over_area(vec2<f32>(emergency_center.x * 0.4, emergency_center.y * 0.3), vec2<f32>(0.12, 0.12));

            var emergency_color: vec4<f32>;
            if (uniforms.safety_emergency_stop < 0.5) {
                // Emergency stop is active - show as resume button (green)
                if (emergency_mouse_over && uniforms.mouse_pressed > 0.5) {
                    emergency_color = vec4<f32>(0.4, 0.9, 0.5, 1.0);
                } else if (emergency_mouse_over) {
                    emergency_color = vec4<f32>(0.35, 0.85, 0.45, 0.98);
                } else {
                    emergency_color = vec4<f32>(0.3, 0.75, 0.4, 0.95);
                }
            } else {
                // Normal operation - show as emergency stop (red)
                let pulse = sin(uniforms.time * 2.0) * 0.1 + 0.9;
                if (emergency_mouse_over && uniforms.mouse_pressed > 0.5) {
                    emergency_color = vec4<f32>(1.0, 0.3, 0.3, 1.0);
                } else if (emergency_mouse_over) {
                    emergency_color = vec4<f32>(0.95, 0.25, 0.25, 0.98);
                } else {
                    emergency_color = vec4<f32>(0.85 * pulse, 0.2, 0.2, 0.95);
                }
            }

            color = emergency_color;

            // Emergency icon
            let icon_pos = (vec2<f32>(local_x, local_y) - emergency_center) / (emergency_radius * 1.5) + vec2<f32>(0.5, 0.5);
            let icon_intensity = draw_icon_pattern(icon_pos, 3);
            if (icon_intensity > 0.5) {
                color = mix(color, vec4<f32>(1.0, 1.0, 1.0, 1.0), 0.8);
            }
        }

        // Safety level indicator (horizontal bar)
        if (local_y > 0.65 && local_y < 0.68 && local_x > 0.1 && local_x < 0.9) {
            let safety_position = (local_x - 0.1) / 0.8;
            let safety_level_normalized = uniforms.ui_safety_level / 4.0;

            if (safety_position <= safety_level_normalized) {
                var safety_color: vec3<f32>;
                if (uniforms.ui_safety_level < 1.5) {
                    safety_color = vec3<f32>(0.3, 0.8, 0.3); // Ultra safe (green)
                } else if (uniforms.ui_safety_level < 2.5) {
                    safety_color = vec3<f32>(0.6, 0.8, 0.3); // Safe (yellow-green)
                } else if (uniforms.ui_safety_level < 3.5) {
                    safety_color = vec3<f32>(0.8, 0.6, 0.2); // Standard (orange)
                } else {
                    safety_color = vec3<f32>(0.8, 0.3, 0.2); // High performance (red)
                }
                color = vec4<f32>(safety_color, 0.9);
            } else {
                color = vec4<f32>(0.2, 0.25, 0.3, 0.7); // Inactive part
            }
        }
    }

    // Status indicators section (0.82 - 1.0)
    if (local_y >= 0.82) {
        // Quality level indicator
        if (local_y > 0.84 && local_y < 0.88 && local_x > 0.05 && local_x < 0.35) {
            let quality_bars = 5.0;
            let bar_width = 0.3 / quality_bars;
            let current_bar = floor((local_x - 0.05) / bar_width);

            if (current_bar <= uniforms.ui_quality_level) {
                let intensity = 0.4 + (current_bar / quality_bars) * 0.6;
                color = vec4<f32>(0.2, 0.4 + intensity * 0.4, 0.8, 0.9);
            } else {
                color = vec4<f32>(0.15, 0.2, 0.25, 0.6);
            }
        }

        // Auto-shader status
        if (local_y > 0.84 && local_y < 0.88 && local_x > 0.4 && local_x < 0.6) {
            if (uniforms.ui_auto_shader > 0.5) {
                color = vec4<f32>(0.3, 0.8, 0.5, 0.9); // Green for auto mode
            } else {
                color = vec4<f32>(0.7, 0.7, 0.3, 0.9); // Yellow for manual mode
            }
        }

        // Current shader index display
        if (local_y > 0.84 && local_y < 0.88 && local_x > 0.65 && local_x < 0.95) {
            let shader_index = i32(uniforms.ui_current_shader_index);
            let shader_color_intensity = 0.5 + (f32(shader_index % 3) * 0.25);
            color = vec4<f32>(shader_color_intensity, 0.4, 0.8 - shader_color_intensity * 0.3, 0.9);
        }

        // Playing status with waveform visualization
        if (local_y > 0.92 && local_y < 0.98 && local_x > 0.1 && local_x < 0.9) {
            if (uniforms.ui_is_playing > 0.5) {
                // Audio-reactive visualization
                let wave_x = local_x * 20.0;
                let wave_amplitude = uniforms.overall_volume * 0.3 + 0.1;
                let wave = sin(wave_x + uniforms.time * 5.0) * wave_amplitude;
                let wave_normalized = (wave + wave_amplitude) / (wave_amplitude * 2.0);

                if (abs((local_y - 0.95) * 10.0) < wave_normalized * 2.0) {
                    color = vec4<f32>(0.3, 0.8, 0.6, 0.9);
                } else {
                    color = vec4<f32>(0.2, 0.4, 0.3, 0.7);
                }
            } else {
                color = vec4<f32>(0.4, 0.4, 0.4, 0.7); // Gray for paused
            }
        }
    }

    // Section dividers with constructivist styling
    if (abs(local_y - 0.18) < 0.001 || abs(local_y - 0.38) < 0.001 ||
        abs(local_y - 0.62) < 0.001 || abs(local_y - 0.82) < 0.001) {
        color = vec4<f32>(0.3, 0.4, 0.5, 0.8);
    }

    // Mouse cursor feedback (subtle highlight)
    let mouse_pos = vec2<f32>(uniforms.mouse_x, uniforms.mouse_y);
    let cursor_distance = distance(screen_pos, mouse_pos);
    if (cursor_distance < 0.012) {
        color = mix(color, vec4<f32>(0.6, 0.7, 0.9, 1.0), 0.2);
    }

    return color;
}