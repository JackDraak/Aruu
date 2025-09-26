use super::{ShaderParameters, Smoother, SmoothingType, Smoothable, PaletteManager};
use crate::audio::{AudioFeatures, RhythmFeatures};

pub struct FeatureMapper {
    smoother: Smoother,
    palette_manager: PaletteManager,
    frame_time: f32,
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
            ("saturation", SmoothingType::exponential(4.0)), // Fast response for volume changes
        ]);

        Self {
            smoother,
            palette_manager: PaletteManager::new(),
            frame_time: 0.0,
        }
    }

    pub fn map_features_to_parameters(&mut self, features: &AudioFeatures) -> ShaderParameters {
        // Update frame time for palette management
        self.frame_time += 1.0 / 60.0; // Assuming 60 FPS

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

        // Calculate saturation based on signal level in dB
        // Near silence (-60dB) = 0.0 saturation, -6dB = 1.0 saturation
        params.saturation = Self::calculate_saturation_from_db(features.signal_level_db);

        // Update transitions
        self.palette_manager.update_transition(self.frame_time);

        // Set current and previous palette information for transition
        let current_palette = self.palette_manager.current_palette();
        let previous_palette = self.palette_manager.previous_palette();
        let transition_blend = self.palette_manager.get_transition_blend(self.frame_time);

        params.palette_index = current_palette.as_index();
        params.palette_base_hue = current_palette.base_hue();
        params.palette_hue_range = current_palette.hue_range();
        params.transition_blend = transition_blend;
        params.prev_palette_index = previous_palette.as_index();
        params.prev_palette_base_hue = previous_palette.base_hue();
        params.prev_palette_hue_range = previous_palette.hue_range();

        // Apply advanced smoothing
        params.apply_smoothing(&mut self.smoother);

        params
    }

    pub fn map_features_with_rhythm(&mut self, features: &AudioFeatures, rhythm: &RhythmFeatures) -> ShaderParameters {
        // Update frame time for palette management
        self.frame_time += 1.0 / 60.0; // Assuming 60 FPS

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

        // Calculate saturation based on signal level in dB
        params.saturation = Self::calculate_saturation_from_db(features.signal_level_db);

        // Try to switch palette on downbeat detection
        self.palette_manager.try_switch_palette(self.frame_time, rhythm.downbeat_detected);

        // Update transitions
        self.palette_manager.update_transition(self.frame_time);

        // Set current and previous palette information for transition
        let current_palette = self.palette_manager.current_palette();
        let previous_palette = self.palette_manager.previous_palette();
        let transition_blend = self.palette_manager.get_transition_blend(self.frame_time);

        params.palette_index = current_palette.as_index();
        params.palette_base_hue = current_palette.base_hue();
        params.palette_hue_range = current_palette.hue_range();
        params.transition_blend = transition_blend;
        params.prev_palette_index = previous_palette.as_index();
        params.prev_palette_base_hue = previous_palette.base_hue();
        params.prev_palette_hue_range = previous_palette.hue_range();

        // Apply advanced smoothing (palette parameters excluded to prevent visual artifacts)
        params.apply_smoothing(&mut self.smoother);

        params
    }

    fn calculate_saturation_from_db(signal_db: f32) -> f32 {
        // Map dB range: -60dB (silence) -> 0.0 saturation, -6dB (peak) -> 1.0 saturation
        // Use exponential curve for more dramatic low-volume desaturation
        let normalized = (signal_db + 60.0) / 54.0; // Convert -60dB to 0, -6dB to 1
        let clamped = normalized.clamp(0.0, 1.0);

        // Apply exponential curve: more gradual at low volumes, steeper at high volumes
        let exponential_curve = clamped * clamped; // Square for exponential effect

        // Ensure complete desaturation below -50dB
        if signal_db < -50.0 {
            0.0
        } else if signal_db < -30.0 {
            // Gradual ramp from -50dB to -30dB
            let ramp = (signal_db + 50.0) / 20.0; // -50dB = 0, -30dB = 1
            (ramp * ramp).clamp(0.0, 1.0) * 0.3 // Max 30% saturation until -30dB
        } else {
            // Full saturation curve from -30dB to -6dB
            exponential_curve
        }
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
            // 5-band frequency analysis
            sub_bass: 0.1,
            bass: 0.5,
            mid: 0.3,
            treble: 0.2,
            presence: 0.1,

            // Volume and dynamics
            overall_volume: 0.4,
            signal_level_db: -12.0,  // Moderate level
            peak_level_db: -6.0,     // Peak level
            dynamic_range: 0.3,

            // Spectral characteristics
            spectral_centroid: 2000.0,
            spectral_rolloff: 8000.0,
            spectral_flux: 0.2,

            // Harmonic and pitch analysis
            pitch_confidence: 0.5,
            zero_crossing_rate: 0.1,

            // Transient detection
            onset_strength: 0.3,
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
            // 5-band frequency analysis
            sub_bass: 0.8,
            bass: 1.0,
            mid: 1.0,
            treble: 1.0,
            presence: 0.9,

            // Volume and dynamics
            overall_volume: 1.0,
            signal_level_db: -6.0,
            peak_level_db: -3.0,
            dynamic_range: 0.8,

            // Spectral characteristics
            spectral_centroid: 1000.0,
            spectral_rolloff: 5000.0,
            spectral_flux: 0.4,

            // Harmonic and pitch analysis
            pitch_confidence: 0.7,
            zero_crossing_rate: 0.1,

            // Transient detection
            onset_strength: 0.6,
        };

        let _params1 = mapper.map_features_to_parameters(&features1);

        let features2 = AudioFeatures {
            // 5-band frequency analysis
            sub_bass: 0.0,
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            presence: 0.0,

            // Volume and dynamics
            overall_volume: 0.0,
            signal_level_db: -40.0,  // Quiet
            peak_level_db: -30.0,
            dynamic_range: 0.1,

            // Spectral characteristics
            spectral_centroid: 2000.0,
            spectral_rolloff: 10000.0,
            spectral_flux: 0.1,

            // Harmonic and pitch analysis
            pitch_confidence: 0.2,
            zero_crossing_rate: 0.2,

            // Transient detection
            onset_strength: 0.1,
        };

        let params2 = mapper.map_features_to_parameters(&features2);

        // With 0.5 smoothing factor, values should be between previous (1.0) and new (0.0)
        assert!(params2.bass_response > 0.0 && params2.bass_response < 1.0);
        assert!(params2.mid_response > 0.0 && params2.mid_response < 1.0);
        assert!(params2.treble_response > 0.0 && params2.treble_response < 1.0);
    }
}