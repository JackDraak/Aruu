use super::{ShaderParameters, Smoother, SmoothingType, Smoothable};
use crate::audio::AudioFeatures;

pub struct FeatureMapper {
    smoother: Smoother,
}

impl FeatureMapper {
    pub fn new() -> Self {
        let mut smoother = Smoother::new();

        // Configure different smoothing types for different parameters
        smoother.configure_multiple(&[
            ("color_intensity", SmoothingType::adaptive(0.05, 0.3, 3.0)),
            ("frequency_scale", SmoothingType::exponential(2.0)),
            ("time_factor", SmoothingType::linear(0.15)),
            ("bass_response", SmoothingType::adaptive(0.1, 0.6, 4.0)), // Fast response for bass
            ("mid_response", SmoothingType::adaptive(0.08, 0.4, 2.5)),
            ("treble_response", SmoothingType::adaptive(0.05, 0.5, 5.0)), // Very responsive for treble
            ("overall_brightness", SmoothingType::exponential(3.0)),
            ("spectral_shift", SmoothingType::exponential(1.5)),
        ]);

        Self {
            smoother,
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

        // Apply advanced smoothing
        params.apply_smoothing(&mut self.smoother);

        params
    }

    pub fn configure_smoothing(&mut self, param_name: &str, smoothing_type: SmoothingType) {
        self.smoother.configure(param_name, smoothing_type);
    }

    pub fn get_change_rate(&self, param_name: &str) -> f32 {
        self.smoother.get_change_rate(param_name)
    }

    pub fn reset_smoothing(&mut self) {
        self.smoother.reset_all();
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

        // Configure for instant response (no smoothing) for testing
        mapper.configure_smoothing("bass_response", SmoothingType::linear(1.0));
        mapper.configure_smoothing("mid_response", SmoothingType::linear(1.0));
        mapper.configure_smoothing("treble_response", SmoothingType::linear(1.0));
        mapper.configure_smoothing("overall_brightness", SmoothingType::linear(1.0));
        mapper.configure_smoothing("color_intensity", SmoothingType::linear(1.0));

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

        assert!((params.bass_response - 0.5).abs() < 0.01);
        assert!((params.mid_response - 0.3).abs() < 0.01);
        assert!((params.treble_response - 0.2).abs() < 0.01);
        assert!((params.overall_brightness - 0.4).abs() < 0.01);

        let expected_color_intensity = (0.5f32 * 0.4 + 0.3 * 0.4 + 0.2 * 0.2).clamp(0.0, 1.0);
        assert!((params.color_intensity - expected_color_intensity).abs() < 0.05);
    }

    #[test]
    fn test_parameter_smoothing() {
        let mut mapper = FeatureMapper::new();

        // Use moderate smoothing for testing
        mapper.configure_smoothing("bass_response", SmoothingType::linear(0.5));
        mapper.configure_smoothing("mid_response", SmoothingType::linear(0.5));
        mapper.configure_smoothing("treble_response", SmoothingType::linear(0.5));

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

        // With 0.5 smoothing factor, values should be between previous (1.0) and new (0.0)
        assert!(params2.bass_response > 0.0 && params2.bass_response < 1.0);
        assert!(params2.mid_response > 0.0 && params2.mid_response < 1.0);
        assert!(params2.treble_response > 0.0 && params2.treble_response < 1.0);
    }
}