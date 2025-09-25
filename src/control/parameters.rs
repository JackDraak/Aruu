use super::smoothing::{Smoother, Smoothable};

#[derive(Debug, Clone)]
pub struct ShaderParameters {
    pub color_intensity: f32,
    pub frequency_scale: f32,
    pub time_factor: f32,
    pub bass_response: f32,
    pub mid_response: f32,
    pub treble_response: f32,
    pub overall_brightness: f32,
    pub spectral_shift: f32,
    pub saturation: f32,          // New: volume-based saturation
    pub palette_index: f32,       // New: current color palette
    pub palette_base_hue: f32,    // New: base hue for current palette
    pub palette_hue_range: f32,   // New: hue range for current palette
    pub transition_blend: f32,    // New: blend factor for palette transitions (0.0 = old, 1.0 = new)
    pub prev_palette_index: f32,  // New: previous palette for transitions
    pub prev_palette_base_hue: f32, // New: previous palette base hue
    pub prev_palette_hue_range: f32, // New: previous palette hue range
}

impl ShaderParameters {
    pub fn new() -> Self {
        Self {
            color_intensity: 1.0,
            frequency_scale: 1.0,
            time_factor: 1.0,
            bass_response: 0.0,
            mid_response: 0.0,
            treble_response: 0.0,
            overall_brightness: 1.0,
            spectral_shift: 0.0,
            saturation: 1.0,          // Full saturation by default
            palette_index: 0.0,       // Rainbow palette by default
            palette_base_hue: 0.0,
            palette_hue_range: 1.0,   // Full range for rainbow
            transition_blend: 1.0,    // No transition by default
            prev_palette_index: 0.0,
            prev_palette_base_hue: 0.0,
            prev_palette_hue_range: 1.0,
        }
    }

    pub fn as_array(&self) -> [f32; 16] {
        [
            self.color_intensity,
            self.frequency_scale,
            self.time_factor,
            self.bass_response,
            self.mid_response,
            self.treble_response,
            self.overall_brightness,
            self.spectral_shift,
            self.saturation,
            self.palette_index,
            self.palette_base_hue,
            self.palette_hue_range,
            self.transition_blend,
            self.prev_palette_index,
            self.prev_palette_base_hue,
            self.prev_palette_hue_range,
        ]
    }
}

impl Smoothable for ShaderParameters {
    fn apply_smoothing(&mut self, smoother: &mut Smoother) {
        let smoothed_values = smoother.smooth_multiple(&[
            ("color_intensity", self.color_intensity),
            ("frequency_scale", self.frequency_scale),
            ("time_factor", self.time_factor),
            ("bass_response", self.bass_response),
            ("mid_response", self.mid_response),
            ("treble_response", self.treble_response),
            ("overall_brightness", self.overall_brightness),
            ("spectral_shift", self.spectral_shift),
            ("saturation", self.saturation),
            // Note: palette parameters are not smoothed to avoid visual artifacts during switches
        ]);

        for (name, value) in smoothed_values {
            match name {
                "color_intensity" => self.color_intensity = value,
                "frequency_scale" => self.frequency_scale = value,
                "time_factor" => self.time_factor = value,
                "bass_response" => self.bass_response = value,
                "mid_response" => self.mid_response = value,
                "treble_response" => self.treble_response = value,
                "overall_brightness" => self.overall_brightness = value,
                "spectral_shift" => self.spectral_shift = value,
                "saturation" => self.saturation = value,
                _ => {}
            }
        }
    }
}