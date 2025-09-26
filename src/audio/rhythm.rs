use std::collections::VecDeque;

const ONSET_THRESHOLD: f32 = 0.1;
const TEMPO_WINDOW_SIZE: usize = 100;
const MIN_BPM: f32 = 60.0;
const MAX_BPM: f32 = 200.0;

#[derive(Debug, Clone)]
pub struct RhythmFeatures {
    pub beat_strength: f32,
    pub tempo_bpm: f32,
    pub estimated_bpm: f32,        // Enhanced BPM estimation with confidence
    pub tempo_confidence: f32,     // Confidence in BPM estimation (0-1)
    pub onset_detected: bool,
    pub rhythm_stability: f32,
    pub downbeat_detected: bool,
    pub beat_position: u8, // 0-3 for quarter notes in 4/4 time
}

impl RhythmFeatures {
    pub fn new() -> Self {
        Self {
            beat_strength: 0.0,
            tempo_bpm: 120.0,
            estimated_bpm: 120.0,
            tempo_confidence: 0.0,
            onset_detected: false,
            rhythm_stability: 0.0,
            downbeat_detected: false,
            beat_position: 0,
        }
    }
}

pub struct RhythmDetector {
    energy_history: VecDeque<f32>,
    onset_times: VecDeque<f32>,
    last_energy: f32,
    frame_count: u64,
    sample_rate: f32,
    beat_counter: u8,
    last_beat_time: f32,
    tempo_stable: bool,
    tempo_history: VecDeque<f32>,   // Track tempo estimates over time
    last_estimated_bpm: f32,
    tempo_confidence: f32,
}

impl RhythmDetector {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            energy_history: VecDeque::with_capacity(TEMPO_WINDOW_SIZE),
            onset_times: VecDeque::with_capacity(50),
            last_energy: 0.0,
            frame_count: 0,
            sample_rate,
            beat_counter: 0,
            last_beat_time: 0.0,
            tempo_stable: false,
            tempo_history: VecDeque::with_capacity(20),
            last_estimated_bpm: 120.0,
            tempo_confidence: 0.0,
        }
    }

    pub fn process_frame(&mut self, frequency_bins: &[f32]) -> RhythmFeatures {
        self.frame_count += 1;
        let current_time = self.frame_count as f32 / 60.0;

        let current_energy = self.calculate_energy(frequency_bins);
        let onset_detected = self.detect_onset(current_energy);

        let mut downbeat_detected = false;
        let mut beat_position = self.beat_counter;

        if onset_detected {
            self.onset_times.push_back(current_time);

            if self.onset_times.len() > 50 {
                self.onset_times.pop_front();
            }

            // Check if this is a strong beat (potential downbeat or beat)
            let tempo_bpm = self.estimate_tempo();
            let expected_beat_interval = 60.0 / tempo_bpm;
            let current_beat_strength = self.calculate_beat_strength(current_energy);

            // If we have established tempo and this onset aligns with expected beat timing
            if self.tempo_stable && (current_time - self.last_beat_time) >= (expected_beat_interval * 0.8) {
                self.beat_counter = (self.beat_counter + 1) % 4;
                beat_position = self.beat_counter;
                self.last_beat_time = current_time;

                // Downbeat is beat position 0 with extra strength requirement
                if self.beat_counter == 0 && current_beat_strength > 0.7 {
                    downbeat_detected = true;
                }
            }
        }

        self.energy_history.push_back(current_energy);
        if self.energy_history.len() > TEMPO_WINDOW_SIZE {
            self.energy_history.pop_front();
        }

        let tempo_bpm = self.estimate_tempo();
        let beat_strength = self.calculate_beat_strength(current_energy);
        let rhythm_stability = self.calculate_rhythm_stability();

        // Enhanced BPM estimation and confidence tracking
        let estimated_bpm = self.estimate_tempo();
        self.update_tempo_confidence(estimated_bpm);

        // Mark tempo as stable if rhythm stability is high
        if rhythm_stability > 0.6 {
            self.tempo_stable = true;
        }

        self.last_energy = current_energy;

        RhythmFeatures {
            beat_strength,
            tempo_bpm,
            estimated_bpm: self.last_estimated_bpm,
            tempo_confidence: self.tempo_confidence,
            onset_detected,
            rhythm_stability,
            downbeat_detected,
            beat_position,
        }
    }

    fn calculate_energy(&self, frequency_bins: &[f32]) -> f32 {
        frequency_bins.iter()
            .take(frequency_bins.len() / 4)
            .map(|&x| x * x)
            .sum::<f32>()
            .sqrt()
    }

    fn detect_onset(&self, current_energy: f32) -> bool {
        if self.energy_history.len() < 10 {
            return false;
        }

        let recent_avg = self.energy_history.iter()
            .rev()
            .take(10)
            .sum::<f32>() / 10.0;

        let energy_increase = current_energy - recent_avg;
        energy_increase > ONSET_THRESHOLD && current_energy > self.last_energy * 1.2
    }

    fn estimate_tempo(&self) -> f32 {
        if self.onset_times.len() < 8 {
            return 120.0; // Need more data for accurate estimation
        }

        let times: Vec<f32> = self.onset_times.iter().copied().collect();
        let mut all_intervals = Vec::new();

        // Collect all inter-onset intervals
        for i in 1..times.len() {
            let interval = times[i] - times[i-1];
            if interval > 0.15 && interval < 2.5 { // Extended reasonable range
                all_intervals.push(interval);
            }
        }

        if all_intervals.len() < 4 {
            return 120.0;
        }

        // Enhanced BPM detection using multiple approaches

        // 1. Histogram-based approach for tempo clustering
        let tempo_candidates = self.find_tempo_candidates(&all_intervals);

        // 2. Autocorrelation-like approach for periodic patterns
        let autocorr_tempo = self.autocorrelation_tempo(&times);

        // 3. Combine results with confidence weighting
        let final_tempo = if tempo_candidates.is_empty() {
            autocorr_tempo
        } else {
            // Weight histogram result more heavily if we have good candidates
            let histogram_tempo = tempo_candidates[0]; // Best candidate
            (histogram_tempo * 0.7 + autocorr_tempo * 0.3)
        };

        final_tempo.clamp(MIN_BPM, MAX_BPM)
    }

    fn find_tempo_candidates(&self, intervals: &[f32]) -> Vec<f32> {
        // Create histogram of BPM values with tolerance
        let mut bpm_counts: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();

        for &interval in intervals {
            let bpm = (60.0 / interval) as u32;
            if bpm >= MIN_BPM as u32 && bpm <= MAX_BPM as u32 {
                *bpm_counts.entry(bpm).or_insert(0) += 1;
                // Also count nearby BPM values to handle slight variations
                *bpm_counts.entry(bpm.saturating_sub(1)).or_insert(0) += 1;
                *bpm_counts.entry(bpm + 1).or_insert(0) += 1;
            }
        }

        // Find most frequent BPM values
        let mut candidates: Vec<(u32, u32)> = bpm_counts.into_iter().collect();
        candidates.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count, descending

        // Return top candidates as f32
        candidates.into_iter()
            .take(3)
            .map(|(bpm, _)| bpm as f32)
            .collect()
    }

    fn autocorrelation_tempo(&self, onset_times: &[f32]) -> f32 {
        if onset_times.len() < 8 {
            return 120.0;
        }

        let time_span = onset_times[onset_times.len()-1] - onset_times[0];
        if time_span < 4.0 { // Need at least 4 seconds of data
            return 120.0;
        }

        // Test different period lengths for periodicity
        let mut best_score = 0.0;
        let mut best_period = 0.5; // 120 BPM default

        // Test periods from 0.3s (200 BPM) to 1.0s (60 BPM)
        for test_period in (30..=100).map(|x| x as f32 / 100.0) {
            let mut score = 0.0;
            let mut count = 0;

            // For each onset, check if there's another onset near the expected periodic time
            for &onset_time in onset_times {
                let mut period_time = onset_time + test_period;
                while period_time <= onset_times[onset_times.len()-1] {
                    // Look for onsets within tolerance of the expected periodic time
                    let tolerance = test_period * 0.1; // 10% tolerance
                    for &other_onset in onset_times {
                        if (other_onset - period_time).abs() < tolerance {
                            score += 1.0 - (other_onset - period_time).abs() / tolerance;
                            break;
                        }
                    }
                    count += 1;
                    period_time += test_period;
                }
            }

            if count > 0 {
                score /= count as f32;
                if score > best_score {
                    best_score = score;
                    best_period = test_period;
                }
            }
        }

        60.0 / best_period
    }

    fn update_tempo_confidence(&mut self, new_estimate: f32) {
        // Add new estimate to history
        self.tempo_history.push_back(new_estimate);
        if self.tempo_history.len() > 20 {
            self.tempo_history.pop_front();
        }

        if self.tempo_history.len() < 5 {
            self.tempo_confidence = 0.1; // Low confidence with little data
            self.last_estimated_bpm = new_estimate;
            return;
        }

        // Calculate confidence based on consistency of recent estimates
        let recent_estimates: Vec<f32> = self.tempo_history.iter().copied().collect();
        let mean_bpm: f32 = recent_estimates.iter().sum::<f32>() / recent_estimates.len() as f32;

        // Calculate variance to measure consistency
        let variance: f32 = recent_estimates.iter()
            .map(|&bpm| (bpm - mean_bpm).powi(2))
            .sum::<f32>() / recent_estimates.len() as f32;

        let std_dev = variance.sqrt();

        // Convert variance to confidence (lower variance = higher confidence)
        // Scale so that std dev of 10 BPM = 50% confidence, std dev of 0 = 100% confidence
        self.tempo_confidence = (1.0 - (std_dev / 20.0)).clamp(0.0, 1.0);

        // Update the estimated BPM with weighted average (more weight to recent estimates)
        let weight_new = 0.3;
        let weight_history = 0.7;
        self.last_estimated_bpm = weight_new * new_estimate + weight_history * mean_bpm;
    }

    fn calculate_beat_strength(&self, current_energy: f32) -> f32 {
        if self.energy_history.len() < 20 {
            return 0.0;
        }

        let recent_max = self.energy_history.iter()
            .rev()
            .take(20)
            .fold(0.0f32, |acc, &x| acc.max(x));

        if recent_max > 0.0 {
            (current_energy / recent_max).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    fn calculate_rhythm_stability(&self) -> f32 {
        if self.onset_times.len() < 8 {
            return 0.0;
        }

        let times: Vec<f32> = self.onset_times.iter().copied().collect();
        let mut intervals = Vec::new();

        for i in 1..times.len() {
            intervals.push(times[i] - times[i-1]);
        }

        if intervals.len() < 4 {
            return 0.0;
        }

        let mean_interval = intervals.iter().sum::<f32>() / intervals.len() as f32;
        let variance = intervals.iter()
            .map(|&x| (x - mean_interval).powi(2))
            .sum::<f32>() / intervals.len() as f32;

        let stability = 1.0 / (1.0 + variance * 10.0);
        stability.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_rhythm_detector_creation() {
        let detector = RhythmDetector::new(44100.0);
        assert_eq!(detector.sample_rate, 44100.0);
        assert_eq!(detector.frame_count, 0);
    }

    #[test]
    fn test_energy_calculation() {
        let detector = RhythmDetector::new(44100.0);
        let bins = vec![1.0, 2.0, 3.0, 4.0];
        let energy = detector.calculate_energy(&bins);
        let expected = (1.0_f32).sqrt();
        assert_abs_diff_eq!(energy, expected, epsilon = 0.001);
    }

    #[test]
    fn test_tempo_estimation() {
        let mut detector = RhythmDetector::new(44100.0);

        for i in 0..10 {
            detector.onset_times.push_back(i as f32 * 0.5);
        }

        let tempo = detector.estimate_tempo();
        assert_abs_diff_eq!(tempo, 120.0, epsilon = 5.0);
    }

    #[test]
    fn test_rhythm_features_default() {
        let features = RhythmFeatures::new();
        assert_eq!(features.tempo_bpm, 120.0);
        assert_eq!(features.estimated_bpm, 120.0);
        assert_eq!(features.tempo_confidence, 0.0);
        assert_eq!(features.onset_detected, false);
        assert_eq!(features.downbeat_detected, false);
        assert_eq!(features.beat_position, 0);
    }
}