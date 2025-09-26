#[derive(Debug, Clone)]
pub struct AudioFeatures {
    // 5-band frequency analysis
    pub sub_bass: f32,        // 20-60 Hz - deep low-end content
    pub bass: f32,            // 60-200 Hz - fundamental bass frequencies
    pub mid: f32,             // 200-2000 Hz - vocal and instrument fundamentals
    pub treble: f32,          // 2000-8000 Hz - clarity and presence
    pub presence: f32,        // 8000+ Hz - air and sparkle

    // Volume and dynamics
    pub overall_volume: f32,
    pub signal_level_db: f32,     // Signal level in dB
    pub peak_level_db: f32,       // Peak level in dB
    pub dynamic_range: f32,       // RMS variation over time

    // Spectral characteristics
    pub spectral_centroid: f32,   // Brightness measure
    pub spectral_rolloff: f32,    // Frequency below which 85% of energy is contained
    pub spectral_flux: f32,       // Frame-to-frame spectral difference

    // Harmonic and pitch analysis
    pub pitch_confidence: f32,    // Harmonic content confidence (0-1)
    pub zero_crossing_rate: f32,  // Rate of sign changes in time domain

    // Transient detection
    pub onset_strength: f32,      // Strength of transient events
}

impl AudioFeatures {
    pub fn new() -> Self {
        Self {
            // 5-band frequency analysis
            sub_bass: 0.0,
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            presence: 0.0,

            // Volume and dynamics
            overall_volume: 0.0,
            signal_level_db: -60.0,  // Very quiet default
            peak_level_db: -60.0,
            dynamic_range: 0.0,

            // Spectral characteristics
            spectral_centroid: 0.0,
            spectral_rolloff: 0.0,
            spectral_flux: 0.0,

            // Harmonic and pitch analysis
            pitch_confidence: 0.0,
            zero_crossing_rate: 0.0,

            // Transient detection
            onset_strength: 0.0,
        }
    }

    pub fn from_frequency_bins(bins: &[f32], sample_rate: f32) -> Self {
        let total_bins = bins.len();
        let nyquist = sample_rate / 2.0;

        // 5-band frequency analysis with precise frequency ranges
        let sub_bass_limit = (60.0 / nyquist * total_bins as f32) as usize;
        let bass_limit = (200.0 / nyquist * total_bins as f32) as usize;
        let mid_limit = (2000.0 / nyquist * total_bins as f32) as usize;
        let treble_limit = (8000.0 / nyquist * total_bins as f32) as usize;

        // Calculate frequency band energies
        let sub_bass = if sub_bass_limit > 0 {
            bins[0..sub_bass_limit.min(total_bins)].iter().sum::<f32>() / sub_bass_limit as f32
        } else {
            0.0
        };

        let bass = if bass_limit > sub_bass_limit {
            bins[sub_bass_limit..bass_limit.min(total_bins)].iter().sum::<f32>()
                / (bass_limit - sub_bass_limit) as f32
        } else {
            0.0
        };

        let mid = if mid_limit > bass_limit {
            bins[bass_limit..mid_limit.min(total_bins)].iter().sum::<f32>()
                / (mid_limit - bass_limit) as f32
        } else {
            0.0
        };

        let treble = if treble_limit > mid_limit {
            bins[mid_limit..treble_limit.min(total_bins)].iter().sum::<f32>()
                / (treble_limit - mid_limit) as f32
        } else {
            0.0
        };

        let presence = if total_bins > treble_limit {
            bins[treble_limit..total_bins].iter().sum::<f32>()
                / (total_bins - treble_limit) as f32
        } else {
            0.0
        };

        let overall_volume = bins.iter().sum::<f32>() / total_bins as f32;

        // Calculate signal levels in dB
        let rms = (bins.iter().map(|x| x * x).sum::<f32>() / total_bins as f32).sqrt();
        let signal_level_db = if rms > 0.0 {
            20.0 * rms.log10()
        } else {
            -60.0
        };

        let peak = bins.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
        let peak_level_db = if peak > 0.0 {
            20.0 * peak.log10()
        } else {
            -60.0
        };

        // Advanced spectral analysis
        let spectral_centroid = Self::calculate_spectral_centroid(bins, sample_rate);
        let spectral_rolloff = Self::calculate_spectral_rolloff(bins, sample_rate);
        let pitch_confidence = Self::calculate_pitch_confidence(bins);
        let onset_strength = Self::calculate_onset_strength(bins);

        Self {
            // 5-band frequency analysis
            sub_bass,
            bass,
            mid,
            treble,
            presence,

            // Volume and dynamics
            overall_volume,
            signal_level_db,
            peak_level_db,
            dynamic_range: 0.0, // TODO: Requires frame-to-frame tracking

            // Spectral characteristics
            spectral_centroid,
            spectral_rolloff,
            spectral_flux: 0.0, // TODO: Requires previous frame data

            // Harmonic and pitch analysis
            pitch_confidence,
            zero_crossing_rate: 0.0, // TODO: Requires time-domain data

            // Transient detection
            onset_strength,
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

    fn calculate_pitch_confidence(bins: &[f32]) -> f32 {
        // Calculate pitch confidence based on harmonic structure
        // Higher values indicate more harmonic/tonal content
        let total_bins = bins.len();

        if total_bins < 8 {
            return 0.0;
        }

        // Look for harmonic peaks (simple approach)
        let mut harmonic_strength = 0.0;
        let mut peak_count = 0;

        // Find peaks in the spectrum
        for i in 2..total_bins-2 {
            if bins[i] > bins[i-1] && bins[i] > bins[i+1] && bins[i] > 0.01 {
                // Check for harmonic relationships (simplified)
                for j in 1..4 { // Check first few harmonics
                    let harmonic_idx = i * (j + 1);
                    if harmonic_idx < total_bins && bins[harmonic_idx] > 0.005 {
                        harmonic_strength += bins[i] * bins[harmonic_idx];
                        peak_count += 1;
                    }
                }
            }
        }

        // Normalize and return confidence
        if peak_count > 0 {
            (harmonic_strength / peak_count as f32).min(1.0)
        } else {
            0.0
        }
    }

    fn calculate_onset_strength(bins: &[f32]) -> f32 {
        // Calculate onset strength based on high-frequency energy and spectral complexity
        let total_bins = bins.len();

        if total_bins < 4 {
            return 0.0;
        }

        // High-frequency energy (above mid-range)
        let hf_start = total_bins / 3; // Roughly above 7kHz at 44.1kHz
        let hf_energy: f32 = bins[hf_start..].iter().sum();
        let total_energy: f32 = bins.iter().sum();

        let hf_ratio = if total_energy > 0.0 {
            hf_energy / total_energy
        } else {
            0.0
        };

        // Spectral complexity (variation in adjacent bins)
        let mut spectral_variation = 0.0;
        for i in 1..total_bins {
            spectral_variation += (bins[i] - bins[i-1]).abs();
        }

        let normalized_variation = spectral_variation / total_bins as f32;

        // Combine high-frequency ratio and spectral complexity
        ((hf_ratio * 2.0 + normalized_variation) / 3.0).min(1.0)
    }
}