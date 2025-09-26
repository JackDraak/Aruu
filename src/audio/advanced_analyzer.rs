use super::AudioFeatures;
use std::collections::VecDeque;

/// Advanced audio analyzer that maintains state between frames for temporal analysis
pub struct AdvancedAudioAnalyzer {
    previous_spectrum: Vec<f32>,
    rms_history: VecDeque<f32>,
    sample_rate: f32,
    frame_count: u64,
    history_size: usize,
}

impl AdvancedAudioAnalyzer {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            previous_spectrum: Vec::new(),
            rms_history: VecDeque::with_capacity(100), // Track ~1.7 seconds at 60fps
            sample_rate,
            frame_count: 0,
            history_size: 100,
        }
    }

    /// Analyze frequency bins with full temporal context
    pub fn analyze_with_context(&mut self, bins: &[f32], time_domain_samples: Option<&[f32]>) -> AudioFeatures {
        self.frame_count += 1;

        // Start with basic analysis from frequency bins
        let mut features = AudioFeatures::from_frequency_bins(bins, self.sample_rate);

        // Calculate spectral flux (frame-to-frame spectral difference)
        features.spectral_flux = self.calculate_spectral_flux(bins);

        // Calculate dynamic range from RMS history
        features.dynamic_range = self.calculate_dynamic_range(&features);

        // Calculate zero crossing rate if time-domain data is available
        if let Some(samples) = time_domain_samples {
            features.zero_crossing_rate = Self::calculate_zero_crossing_rate(samples);
        }

        // Update state for next frame
        self.update_state(bins, &features);

        features
    }

    fn calculate_spectral_flux(&self, current_spectrum: &[f32]) -> f32 {
        if self.previous_spectrum.is_empty() || self.previous_spectrum.len() != current_spectrum.len() {
            return 0.0; // No previous frame to compare
        }

        let mut flux = 0.0;
        let mut total_energy = 0.0;

        for (i, (&current, &previous)) in current_spectrum.iter().zip(self.previous_spectrum.iter()).enumerate() {
            // Calculate positive spectral difference (only increases in energy)
            let diff = (current - previous).max(0.0);
            flux += diff * diff;
            total_energy += current * current;
        }

        // Normalize by total spectral energy to get relative flux
        if total_energy > 0.0 {
            (flux / total_energy).sqrt().min(1.0)
        } else {
            0.0
        }
    }

    fn calculate_dynamic_range(&mut self, features: &AudioFeatures) -> f32 {
        // Add current RMS to history
        let current_rms = 10.0_f32.powf(features.signal_level_db / 20.0); // Convert dB back to linear
        self.rms_history.push_back(current_rms);

        // Maintain history size
        while self.rms_history.len() > self.history_size {
            self.rms_history.pop_front();
        }

        // Calculate dynamic range as the variance in RMS over the recent history
        if self.rms_history.len() < 10 {
            return 0.0; // Need some history
        }

        let mean_rms: f32 = self.rms_history.iter().sum::<f32>() / self.rms_history.len() as f32;
        let variance: f32 = self.rms_history.iter()
            .map(|&rms| (rms - mean_rms).powi(2))
            .sum::<f32>() / self.rms_history.len() as f32;

        // Convert variance to a normalized dynamic range measure
        variance.sqrt().min(1.0)
    }

    fn calculate_zero_crossing_rate(samples: &[f32]) -> f32 {
        if samples.len() < 2 {
            return 0.0;
        }

        let mut zero_crossings = 0;
        for window in samples.windows(2) {
            if (window[0] > 0.0) != (window[1] > 0.0) {
                zero_crossings += 1;
            }
        }

        // Normalize by sample count and typical expected range
        let rate = zero_crossings as f32 / (samples.len() - 1) as f32;
        (rate * 10.0).min(1.0) // Scale to reasonable range
    }

    fn update_state(&mut self, current_spectrum: &[f32], _features: &AudioFeatures) {
        // Store current spectrum for next frame's flux calculation
        self.previous_spectrum.clear();
        self.previous_spectrum.extend_from_slice(current_spectrum);
    }

    /// Reset analyzer state (useful when switching audio sources)
    pub fn reset(&mut self) {
        self.previous_spectrum.clear();
        self.rms_history.clear();
        self.frame_count = 0;
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_analyzer_creation() {
        let analyzer = AdvancedAudioAnalyzer::new(44100.0);
        assert_eq!(analyzer.sample_rate, 44100.0);
        assert_eq!(analyzer.frame_count(), 0);
    }

    #[test]
    fn test_spectral_flux_calculation() {
        let mut analyzer = AdvancedAudioAnalyzer::new(44100.0);

        // First frame - should have zero flux (no previous frame)
        let bins1 = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let features1 = analyzer.analyze_with_context(&bins1, None);
        assert_eq!(features1.spectral_flux, 0.0);

        // Second frame - should have some flux
        let bins2 = vec![0.2, 0.4, 0.6, 0.8, 1.0]; // Increased energy
        let features2 = analyzer.analyze_with_context(&bins2, None);
        assert!(features2.spectral_flux > 0.0);
        assert!(features2.spectral_flux <= 1.0);
    }

    #[test]
    fn test_zero_crossing_rate() {
        // Create a simple sine-like pattern
        let samples = vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0];
        let zcr = AdvancedAudioAnalyzer::calculate_zero_crossing_rate(&samples);

        assert!(zcr > 0.0);
        assert!(zcr <= 1.0);
    }

    #[test]
    fn test_dynamic_range_tracking() {
        let mut analyzer = AdvancedAudioAnalyzer::new(44100.0);

        // Generate several frames with varying energy
        for level_db in [-30.0, -20.0, -10.0, -40.0, -15.0, -25.0, -5.0] {
            let energy = 10.0_f32.powf(level_db / 20.0);
            let bins: Vec<f32> = (0..512).map(|_| energy * 0.1).collect();
            let features = analyzer.analyze_with_context(&bins, None);

            // Dynamic range should increase as we add more varied frames
            assert!(features.dynamic_range >= 0.0);
            assert!(features.dynamic_range <= 1.0);
        }
    }
}