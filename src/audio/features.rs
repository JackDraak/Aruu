#[derive(Debug, Clone)]
pub struct AudioFeatures {
    pub bass: f32,
    pub mid: f32,
    pub treble: f32,
    pub overall_volume: f32,
    pub signal_level_db: f32,     // Signal level in dB
    pub peak_level_db: f32,       // Peak level in dB
    pub spectral_centroid: f32,
    pub spectral_rolloff: f32,
    pub zero_crossing_rate: f32,
}

impl AudioFeatures {
    pub fn new() -> Self {
        Self {
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            overall_volume: 0.0,
            signal_level_db: -60.0,  // Very quiet default
            peak_level_db: -60.0,
            spectral_centroid: 0.0,
            spectral_rolloff: 0.0,
            zero_crossing_rate: 0.0,
        }
    }

    pub fn from_frequency_bins(bins: &[f32], sample_rate: f32) -> Self {
        let total_bins = bins.len();
        let nyquist = sample_rate / 2.0;

        let bass_limit = (200.0 / nyquist * total_bins as f32) as usize;
        let mid_limit = (2000.0 / nyquist * total_bins as f32) as usize;

        let bass = bins[0..bass_limit.min(total_bins)].iter().sum::<f32>() / bass_limit as f32;
        let mid = bins[bass_limit..mid_limit.min(total_bins)].iter().sum::<f32>()
                  / (mid_limit - bass_limit) as f32;
        let treble = bins[mid_limit..total_bins].iter().sum::<f32>()
                     / (total_bins - mid_limit) as f32;

        let overall_volume = bins.iter().sum::<f32>() / total_bins as f32;

        // Calculate signal levels in dB
        let rms = (bins.iter().map(|x| x * x).sum::<f32>() / total_bins as f32).sqrt();
        let signal_level_db = if rms > 0.0 {
            20.0 * rms.log10()
        } else {
            -60.0 // Very quiet
        };

        let peak = bins.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
        let peak_level_db = if peak > 0.0 {
            20.0 * peak.log10()
        } else {
            -60.0
        };

        let spectral_centroid = Self::calculate_spectral_centroid(bins, sample_rate);
        let spectral_rolloff = Self::calculate_spectral_rolloff(bins, sample_rate);

        Self {
            bass,
            mid,
            treble,
            overall_volume,
            signal_level_db,
            peak_level_db,
            spectral_centroid,
            spectral_rolloff,
            zero_crossing_rate: 0.0,
        }
    }

    fn calculate_spectral_centroid(bins: &[f32], sample_rate: f32) -> f32 {
        let mut weighted_sum = 0.0;
        let mut magnitude_sum = 0.0;

        for (i, &magnitude) in bins.iter().enumerate() {
            let frequency = i as f32 * sample_rate / (2.0 * bins.len() as f32);
            weighted_sum += frequency * magnitude;
            magnitude_sum += magnitude;
        }

        if magnitude_sum > 0.0 {
            weighted_sum / magnitude_sum
        } else {
            0.0
        }
    }

    fn calculate_spectral_rolloff(bins: &[f32], sample_rate: f32) -> f32 {
        let total_energy: f32 = bins.iter().sum();
        let rolloff_threshold = 0.85 * total_energy;
        let mut cumulative_energy = 0.0;

        for (i, &magnitude) in bins.iter().enumerate() {
            cumulative_energy += magnitude;
            if cumulative_energy >= rolloff_threshold {
                return i as f32 * sample_rate / (2.0 * bins.len() as f32);
            }
        }

        sample_rate / 2.0
    }
}