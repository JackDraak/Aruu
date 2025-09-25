use super::{ShaderParameters};
use crate::audio::AudioFeatures;

pub struct FeatureMapper {
    smoothing_factor: f32,
    previous_params: ShaderParameters,
}

impl FeatureMapper {
    pub fn new() -> Self {
        Self {
            smoothing_factor: 0.1,
            previous_params: ShaderParameters::new(),
        }
    }

    pub fn map_features_to_parameters(&mut self, features: &AudioFeatures) -> ShaderParameters {
        let mut params = ShaderParameters::new();

        params.bass_response = features.bass.clamp(0.0, 1.0);
        params.mid_response = features.mid.clamp(0.0, 1.0);
        params.treble_response = features.treble.clamp(0.0, 1.0);

        params.overall_brightness = features.overall_volume.clamp(0.0, 1.0);

        params.color_intensity = (features.bass * 0.4 + features.mid * 0.4 + features.treble * 0.2).clamp(0.0, 1.0);

        params.frequency_scale = 1.0 + features.spectral_centroid / 10000.0;
        params.frequency_scale = params.frequency_scale.clamp(0.5, 2.0);

        params.spectral_shift = (features.spectral_rolloff / 22050.0 - 0.5) * 2.0;
        params.spectral_shift = params.spectral_shift.clamp(-1.0, 1.0);

        params.time_factor = 1.0 + features.overall_volume * 0.5;

        params = self.smooth_parameters(params);

        self.previous_params = params.clone();
        params
    }

    fn smooth_parameters(&self, new_params: ShaderParameters) -> ShaderParameters {
        let prev = &self.previous_params;
        let factor = self.smoothing_factor;

        ShaderParameters {
            color_intensity: lerp(prev.color_intensity, new_params.color_intensity, factor),
            frequency_scale: lerp(prev.frequency_scale, new_params.frequency_scale, factor),
            time_factor: lerp(prev.time_factor, new_params.time_factor, factor),
            bass_response: lerp(prev.bass_response, new_params.bass_response, factor),
            mid_response: lerp(prev.mid_response, new_params.mid_response, factor),
            treble_response: lerp(prev.treble_response, new_params.treble_response, factor),
            overall_brightness: lerp(prev.overall_brightness, new_params.overall_brightness, factor),
            spectral_shift: lerp(prev.spectral_shift, new_params.spectral_shift, factor),
        }
    }

    pub fn set_smoothing_factor(&mut self, factor: f32) {
        self.smoothing_factor = factor.clamp(0.0, 1.0);
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_mapping() {
        let mut mapper = FeatureMapper::new();
        mapper.set_smoothing_factor(1.0);
        let features = AudioFeatures {
            bass: 0.5,
            mid: 0.3,
            treble: 0.2,
            overall_volume: 0.4,
            spectral_centroid: 2000.0,
            spectral_rolloff: 8000.0,
            zero_crossing_rate: 0.1,
        };

        let params = mapper.map_features_to_parameters(&features);

        assert!((params.bass_response - 0.5).abs() < 0.001);
        assert!((params.mid_response - 0.3).abs() < 0.001);
        assert!((params.treble_response - 0.2).abs() < 0.001);
        assert!((params.overall_brightness - 0.4).abs() < 0.001);

        let expected_color_intensity = (0.5f32 * 0.4 + 0.3 * 0.4 + 0.2 * 0.2).clamp(0.0, 1.0);
        assert!((params.color_intensity - expected_color_intensity).abs() < 0.001);
    }

    #[test]
    fn test_parameter_smoothing() {
        let mut mapper = FeatureMapper::new();
        mapper.set_smoothing_factor(0.5);

        let features1 = AudioFeatures {
            bass: 1.0,
            mid: 1.0,
            treble: 1.0,
            overall_volume: 1.0,
            spectral_centroid: 1000.0,
            spectral_rolloff: 5000.0,
            zero_crossing_rate: 0.1,
        };

        let _params1 = mapper.map_features_to_parameters(&features1);

        let features2 = AudioFeatures {
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            overall_volume: 0.0,
            spectral_centroid: 2000.0,
            spectral_rolloff: 10000.0,
            zero_crossing_rate: 0.2,
        };

        let params2 = mapper.map_features_to_parameters(&features2);

        assert!(params2.bass_response > 0.0 && params2.bass_response < 1.0);
        assert!(params2.mid_response > 0.0 && params2.mid_response < 1.0);
        assert!(params2.treble_response > 0.0 && params2.treble_response < 1.0);
    }
}