use std::collections::VecDeque;

const ONSET_THRESHOLD: f32 = 0.1;
const TEMPO_WINDOW_SIZE: usize = 100;
const MIN_BPM: f32 = 60.0;
const MAX_BPM: f32 = 200.0;

#[derive(Debug, Clone)]
pub struct RhythmFeatures {
    pub beat_strength: f32,
    pub tempo_bpm: f32,
    pub onset_detected: bool,
    pub rhythm_stability: f32,
}

impl RhythmFeatures {
    pub fn new() -> Self {
        Self {
            beat_strength: 0.0,
            tempo_bpm: 120.0,
            onset_detected: false,
            rhythm_stability: 0.0,
        }
    }
}

pub struct RhythmDetector {
    energy_history: VecDeque<f32>,
    onset_times: VecDeque<f32>,
    last_energy: f32,
    frame_count: u64,
    sample_rate: f32,
}

impl RhythmDetector {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            energy_history: VecDeque::with_capacity(TEMPO_WINDOW_SIZE),
            onset_times: VecDeque::with_capacity(50),
            last_energy: 0.0,
            frame_count: 0,
            sample_rate,
        }
    }

    pub fn process_frame(&mut self, frequency_bins: &[f32]) -> RhythmFeatures {
        self.frame_count += 1;

        let current_energy = self.calculate_energy(frequency_bins);
        let onset_detected = self.detect_onset(current_energy);

        if onset_detected {
            let current_time = self.frame_count as f32 / 60.0;
            self.onset_times.push_back(current_time);

            if self.onset_times.len() > 50 {
                self.onset_times.pop_front();
            }
        }

        self.energy_history.push_back(current_energy);
        if self.energy_history.len() > TEMPO_WINDOW_SIZE {
            self.energy_history.pop_front();
        }

        let tempo_bpm = self.estimate_tempo();
        let beat_strength = self.calculate_beat_strength(current_energy);
        let rhythm_stability = self.calculate_rhythm_stability();

        self.last_energy = current_energy;

        RhythmFeatures {
            beat_strength,
            tempo_bpm,
            onset_detected,
            rhythm_stability,
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
        if self.onset_times.len() < 4 {
            return 120.0;
        }

        let mut intervals = Vec::new();
        let times: Vec<f32> = self.onset_times.iter().copied().collect();

        for i in 1..times.len() {
            let interval = times[i] - times[i-1];
            if interval > 0.1 && interval < 2.0 {
                intervals.push(interval);
            }
        }

        if intervals.is_empty() {
            return 120.0;
        }

        intervals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_interval = intervals[intervals.len() / 2];

        let bpm = 60.0 / median_interval;
        bpm.clamp(MIN_BPM, MAX_BPM)
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
        assert_eq!(features.onset_detected, false);
    }
}