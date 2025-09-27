// Debug overlay fragment shader - renders performance and audio data on right side

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

// Helper functions for clean geometric text simulation
fn draw_digit(pos: vec2<f32>, digit: i32, size: f32) -> f32 {
    let grid_pos = pos / size;
    let x = i32(grid_pos.x * 5.0);
    let y = i32(grid_pos.y * 7.0);

    if (x < 0 || x >= 5 || y < 0 || y >= 7) {
        return 0.0;
    }

    // Simple 5x7 bitmap font data for digits 0-9 (each row is 5 bits)
    let digit_0 = array<u32, 7>(0x1Fu, 0x11u, 0x11u, 0x11u, 0x11u, 0x11u, 0x1Fu); // 0
    let digit_1 = array<u32, 7>(0x04u, 0x0Cu, 0x04u, 0x04u, 0x04u, 0x04u, 0x1Fu); // 1
    let digit_2 = array<u32, 7>(0x1Fu, 0x01u, 0x01u, 0x1Fu, 0x10u, 0x10u, 0x1Fu); // 2
    let digit_3 = array<u32, 7>(0x1Fu, 0x01u, 0x01u, 0x1Fu, 0x01u, 0x01u, 0x1Fu); // 3
    let digit_4 = array<u32, 7>(0x11u, 0x11u, 0x11u, 0x1Fu, 0x01u, 0x01u, 0x01u); // 4
    let digit_5 = array<u32, 7>(0x1Fu, 0x10u, 0x10u, 0x1Fu, 0x01u, 0x01u, 0x1Fu); // 5
    let digit_6 = array<u32, 7>(0x1Fu, 0x10u, 0x10u, 0x1Fu, 0x11u, 0x11u, 0x1Fu); // 6
    let digit_7 = array<u32, 7>(0x1Fu, 0x01u, 0x01u, 0x01u, 0x01u, 0x01u, 0x01u); // 7
    let digit_8 = array<u32, 7>(0x1Fu, 0x11u, 0x11u, 0x1Fu, 0x11u, 0x11u, 0x1Fu); // 8
    let digit_9 = array<u32, 7>(0x1Fu, 0x11u, 0x11u, 0x1Fu, 0x01u, 0x01u, 0x1Fu); // 9

    if (digit >= 0 && digit <= 9) {
        var pattern: array<u32, 7>;

        if (digit == 0) { pattern = digit_0; }
        else if (digit == 1) { pattern = digit_1; }
        else if (digit == 2) { pattern = digit_2; }
        else if (digit == 3) { pattern = digit_3; }
        else if (digit == 4) { pattern = digit_4; }
        else if (digit == 5) { pattern = digit_5; }
        else if (digit == 6) { pattern = digit_6; }
        else if (digit == 7) { pattern = digit_7; }
        else if (digit == 8) { pattern = digit_8; }
        else { pattern = digit_9; } // digit == 9

        let bit = (pattern[y] >> u32(4 - x)) & 1u;
        return f32(bit);
    }

    return 0.0;
}

fn draw_text_pattern(pos: vec2<f32>, scale: f32) -> f32 {
    // Simple geometric pattern for text simulation
    let grid_x = floor(pos.x / scale) * scale;
    let grid_y = floor(pos.y / scale) * scale;
    let cell_x = (pos.x - grid_x) / scale;
    let cell_y = (pos.y - grid_y) / scale;

    // Create readable geometric patterns
    if (cell_x > 0.1 && cell_x < 0.9 && cell_y > 0.2 && cell_y < 0.8) {
        return 1.0;
    }
    if (cell_x > 0.3 && cell_x < 0.7 && cell_y > 0.0 && cell_y < 1.0) {
        return 0.7;
    }

    return 0.0;
}

fn sdf_rounded_box(pos: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(pos) - size + radius;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - radius;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let screen_pos = input.screen_pos;

    // Only render on right side (x > 0.7)
    if (screen_pos.x < 0.7) {
        discard;
    }

    // Local coordinates within the debug overlay (0.0 to 1.0)
    let local_x = (screen_pos.x - 0.7) / 0.3;
    let local_y = screen_pos.y;

    // Semi-transparent white background for text contrast
    var color = vec4<f32>(1.0, 1.0, 1.0, 0.85);

    // Panel border with subtle gradient
    let border_sdf = sdf_rounded_box(vec2<f32>(local_x - 0.5, local_y - 0.5), vec2<f32>(0.48, 0.48), 0.02);
    if (border_sdf > -0.005 && border_sdf < 0.0) {
        color = vec4<f32>(0.8, 0.8, 0.9, 0.9);
    }

    // Header section (0.0 - 0.15)
    if (local_y < 0.15) {
        color = vec4<f32>(0.9, 0.9, 0.95, 0.9);

        // Header title pattern
        if (local_y > 0.05 && local_y < 0.12 && local_x > 0.1 && local_x < 0.9) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 8.0, (local_y - 0.05) * 20.0), 0.15);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.2, 0.3, 0.5, 0.9);
            }
        }

        // Header underline
        if (abs(local_y - 0.13) < 0.002) {
            color = vec4<f32>(0.4, 0.5, 0.7, 0.9);
        }
    }

    // Audio frequency section (0.15 - 0.45)
    if (local_y >= 0.15 && local_y < 0.45) {
        // Section label area
        if (local_y > 0.16 && local_y < 0.22 && local_x > 0.05 && local_x < 0.95) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 10.0, (local_y - 0.16) * 25.0), 0.12);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.1, 0.2, 0.4, 0.9);
            }
        }

        // 5-band frequency visualization
        let bar_y_start = 0.25;
        let bar_height = 0.15;
        let bar_width = 0.12;
        let bar_spacing = 0.14;

        // Sub-bass bar (deep red)
        let sub_bass_x = 0.08;
        if (local_x >= sub_bass_x && local_x < sub_bass_x + bar_width &&
            local_y >= bar_y_start && local_y < bar_y_start + bar_height * uniforms.sub_bass) {
            color = vec4<f32>(0.8, 0.1, 0.1, 0.95);
        }

        // Bass bar (red)
        let bass_x = sub_bass_x + bar_spacing;
        if (local_x >= bass_x && local_x < bass_x + bar_width &&
            local_y >= bar_y_start && local_y < bar_y_start + bar_height * uniforms.bass) {
            color = vec4<f32>(0.9, 0.3, 0.2, 0.95);
        }

        // Mid bar (green)
        let mid_x = bass_x + bar_spacing;
        if (local_x >= mid_x && local_x < mid_x + bar_width &&
            local_y >= bar_y_start && local_y < bar_y_start + bar_height * uniforms.mid) {
            color = vec4<f32>(0.2, 0.8, 0.3, 0.95);
        }

        // Treble bar (blue)
        let treble_x = mid_x + bar_spacing;
        if (local_x >= treble_x && local_x < treble_x + bar_width &&
            local_y >= bar_y_start && local_y < bar_y_start + bar_height * uniforms.treble) {
            color = vec4<f32>(0.2, 0.4, 0.9, 0.95);
        }

        // Presence bar (purple)
        let presence_x = treble_x + bar_spacing;
        if (local_x >= presence_x && local_x < presence_x + bar_width &&
            local_y >= bar_y_start && local_y < bar_y_start + bar_height * uniforms.presence) {
            color = vec4<f32>(0.7, 0.2, 0.8, 0.95);
        }

        // Bar labels (simplified geometric patterns)
        let label_y = 0.42;
        if (local_y > label_y - 0.01 && local_y < label_y + 0.01) {
            if (local_x >= sub_bass_x && local_x < sub_bass_x + bar_width) {
                color = vec4<f32>(0.5, 0.1, 0.1, 0.9);
            } else if (local_x >= bass_x && local_x < bass_x + bar_width) {
                color = vec4<f32>(0.6, 0.2, 0.1, 0.9);
            } else if (local_x >= mid_x && local_x < mid_x + bar_width) {
                color = vec4<f32>(0.1, 0.5, 0.2, 0.9);
            } else if (local_x >= treble_x && local_x < treble_x + bar_width) {
                color = vec4<f32>(0.1, 0.2, 0.6, 0.9);
            } else if (local_x >= presence_x && local_x < presence_x + bar_width) {
                color = vec4<f32>(0.4, 0.1, 0.5, 0.9);
            }
        }
    }

    // BPM section (0.45 - 0.65)
    if (local_y >= 0.45 && local_y < 0.65) {
        // BPM label
        if (local_y > 0.46 && local_y < 0.52 && local_x > 0.05 && local_x < 0.3) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 12.0, (local_y - 0.46) * 25.0), 0.1);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.1, 0.2, 0.4, 0.9);
            }
        }

        // BPM pulsing indicator
        let bpm_center = vec2<f32>(0.5, 0.55);
        let bpm_distance = distance(vec2<f32>(local_x, local_y), bpm_center);
        let bpm_pulse = sin(uniforms.time * uniforms.estimated_bpm * 0.1047) * 0.5 + 0.5; // 60/bpm conversion
        let bpm_radius = 0.06 + bpm_pulse * 0.03;

        if (bpm_distance < bpm_radius) {
            let bpm_intensity = 1.0 - (bpm_distance / bpm_radius);
            let pulse_color = vec4<f32>(1.0, 0.7, 0.2, 0.8 + bpm_pulse * 0.2);
            color = mix(color, pulse_color, bpm_intensity * 0.9);
        }

        // BPM value display (geometric number pattern)
        if (local_y > 0.57 && local_y < 0.62 && local_x > 0.7 && local_x < 0.95) {
            let bpm_hundreds = i32(uniforms.estimated_bpm) / 100;
            let bpm_tens = (i32(uniforms.estimated_bpm) % 100) / 10;
            let bpm_ones = i32(uniforms.estimated_bpm) % 10;

            let char_width = 0.08;
            var digit_intensity = 0.0;

            if (local_x >= 0.70 && local_x < 0.70 + char_width) {
                digit_intensity = draw_digit(vec2<f32>((local_x - 0.70) / char_width, (local_y - 0.57) / 0.05), bpm_hundreds, 1.0);
            } else if (local_x >= 0.78 && local_x < 0.78 + char_width) {
                digit_intensity = draw_digit(vec2<f32>((local_x - 0.78) / char_width, (local_y - 0.57) / 0.05), bpm_tens, 1.0);
            } else if (local_x >= 0.86 && local_x < 0.86 + char_width) {
                digit_intensity = draw_digit(vec2<f32>((local_x - 0.86) / char_width, (local_y - 0.57) / 0.05), bpm_ones, 1.0);
            }

            if (digit_intensity > 0.5) {
                color = vec4<f32>(0.2, 0.3, 0.6, 0.9);
            }
        }
    }

    // Performance section (0.65 - 0.85)
    if (local_y >= 0.65 && local_y < 0.85) {
        // FPS label
        if (local_y > 0.66 && local_y < 0.72 && local_x > 0.05 && local_x < 0.25) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 15.0, (local_y - 0.66) * 25.0), 0.08);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.1, 0.2, 0.4, 0.9);
            }
        }

        // FPS section with clear indicator
        if (local_y > 0.70 && local_y < 0.78) {
            // Simple "F" indicator - just a clear rectangular pattern
            if (local_x > 0.05 && local_x < 0.12 && local_y > 0.72 && local_y < 0.76) {
                // Simple F shape using clear rectangles
                if ((local_x > 0.055 && local_x < 0.065) || // Vertical bar
                    (local_y > 0.745 && local_y < 0.755 && local_x < 0.11) || // Top bar
                    (local_y > 0.725 && local_y < 0.735 && local_x < 0.09)) { // Middle bar
                    color = vec4<f32>(0.3, 0.5, 0.8, 0.95);
                }
            }

            // FPS value display (use existing digit function)
            if (local_y > 0.715 && local_y < 0.765 && local_x > 0.15 && local_x < 0.25) {
                let fps_int = i32(uniforms.ui_fps);
                let fps_tens = fps_int / 10;
                let fps_ones = fps_int % 10;
                let char_width = 0.04;
                var digit_intensity = 0.0;

                if (local_x >= 0.16 && local_x < 0.16 + char_width) {
                    digit_intensity = draw_digit(vec2<f32>((local_x - 0.16) / char_width, (local_y - 0.715) / 0.05), fps_tens, 1.0);
                } else if (local_x >= 0.21 && local_x < 0.21 + char_width) {
                    digit_intensity = draw_digit(vec2<f32>((local_x - 0.21) / char_width, (local_y - 0.715) / 0.05), fps_ones, 1.0);
                }

                if (digit_intensity > 0.5) {
                    var fps_color = vec3<f32>(0.2, 0.8, 0.3); // Green for >58 FPS
                    if (uniforms.ui_fps < 58.0) {
                        fps_color = vec3<f32>(0.8, 0.8, 0.2); // Yellow for 45-58 FPS
                    }
                    if (uniforms.ui_fps < 45.0) {
                        fps_color = vec3<f32>(0.8, 0.2, 0.2); // Red for <45 FPS
                    }
                    color = vec4<f32>(fps_color, 0.95);
                }
            }
        }

        // Frame time section with clear indicator
        if (local_y > 0.78 && local_y < 0.86) {
            // Simple "T" indicator for Time
            if (local_x > 0.05 && local_x < 0.12 && local_y > 0.80 && local_y < 0.84) {
                // Simple T shape using clear rectangles
                if ((local_y > 0.825 && local_y < 0.835) || // Top bar
                    (local_x > 0.08 && local_x < 0.09 && local_y > 0.805)) { // Vertical bar
                    color = vec4<f32>(0.3, 0.5, 0.8, 0.95);
                }
            }

            // Frame time value display (use existing digit function)
            if (local_y > 0.795 && local_y < 0.845 && local_x > 0.15 && local_x < 0.25) {
                let frame_ms = i32(uniforms.ui_frame_time);
                let frame_tens = frame_ms / 10;
                let frame_ones = frame_ms % 10;
                let char_width = 0.04;
                var digit_intensity = 0.0;

                if (local_x >= 0.16 && local_x < 0.16 + char_width) {
                    digit_intensity = draw_digit(vec2<f32>((local_x - 0.16) / char_width, (local_y - 0.795) / 0.05), frame_tens, 1.0);
                } else if (local_x >= 0.21 && local_x < 0.21 + char_width) {
                    digit_intensity = draw_digit(vec2<f32>((local_x - 0.21) / char_width, (local_y - 0.795) / 0.05), frame_ones, 1.0);
                }

                if (digit_intensity > 0.5) {
                    var time_color = vec3<f32>(0.2, 0.6, 0.2);
                    if (frame_ms > 16) {
                        time_color = vec3<f32>(0.6, 0.6, 0.2);
                    }
                    if (frame_ms > 22) {
                        time_color = vec3<f32>(0.6, 0.2, 0.2);
                    }
                    color = vec4<f32>(time_color, 0.95);
                }
            }
        }
    }

    // Safety status section (0.85 - 1.0)
    if (local_y >= 0.85) {
        // Safety label
        if (local_y > 0.86 && local_y < 0.92 && local_x > 0.05 && local_x < 0.35) {
            let text_intensity = draw_text_pattern(vec2<f32>(local_x * 10.0, (local_y - 0.86) * 25.0), 0.1);
            if (text_intensity > 0.5) {
                color = vec4<f32>(0.1, 0.2, 0.4, 0.9);
            }
        }

        // Safety level indicator with color coding
        if (local_y > 0.93 && local_y < 0.98 && local_x > 0.1 && local_x < 0.9) {
            var safety_color = vec3<f32>(0.2, 0.8, 0.3); // Green for safe

            if (uniforms.ui_safety_level >= 2.0) {
                safety_color = vec3<f32>(0.6, 0.8, 0.3); // Yellow-green for moderate
            }
            if (uniforms.ui_safety_level >= 3.0) {
                safety_color = vec3<f32>(0.8, 0.6, 0.2); // Orange for standard
            }
            if (uniforms.ui_safety_level >= 4.0) {
                safety_color = vec3<f32>(0.8, 0.3, 0.2); // Red for higher levels
            }
            if (uniforms.safety_emergency_stop < 0.5) {
                safety_color = vec3<f32>(0.9, 0.1, 0.1); // Bright red for emergency stop
            }

            color = vec4<f32>(safety_color, 0.9);
        }
    }

    // Section dividers
    if (abs(local_y - 0.15) < 0.001 || abs(local_y - 0.45) < 0.001 ||
        abs(local_y - 0.65) < 0.001 || abs(local_y - 0.85) < 0.001) {
        color = vec4<f32>(0.6, 0.6, 0.7, 0.7);
    }

    // Mouse hover highlight
    let mouse_pos = vec2<f32>(uniforms.mouse_x, uniforms.mouse_y);
    let cursor_distance = distance(screen_pos, mouse_pos);
    if (cursor_distance < 0.008) {
        color = mix(color, vec4<f32>(0.2, 0.3, 0.5, 1.0), 0.3);
    }

    return color;
}