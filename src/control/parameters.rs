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
}

impl ShaderParameters {
    pub fn new() -> Self {
        Self {
            color_intensity: 1.0,
            frequency_scale: 1.0,
            time_factor: 1.0,
            bass_response: 1.0,
            mid_response: 1.0,
            treble_response: 1.0,
            overall_brightness: 1.0,
            spectral_shift: 0.0,
        }
    }

    pub fn as_array(&self) -> [f32; 8] {
        [
            self.color_intensity,
            self.frequency_scale,
            self.time_factor,
            self.bass_response,
            self.mid_response,
            self.treble_response,
            self.overall_brightness,
            self.spectral_shift,
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
                _ => {}
            }
        }
    }
}