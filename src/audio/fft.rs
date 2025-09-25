use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::Arc;

pub struct FftAnalyzer {
    fft: Arc<dyn rustfft::Fft<f32>>,
    buffer: Vec<Complex<f32>>,
    window: Vec<f32>,
    scratch: Vec<Complex<f32>>,
    output_buffer: Vec<f32>,
}

impl FftAnalyzer {
    pub fn new(size: usize) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(size);
        let scratch_len = fft.get_inplace_scratch_len();

        let buffer = vec![Complex::new(0.0, 0.0); size];
        let window = Self::hann_window(size);
        let scratch = vec![Complex::new(0.0, 0.0); scratch_len];
        let output_buffer = vec![0.0; size / 2];

        Self {
            fft,
            buffer,
            window,
            scratch,
            output_buffer,
        }
    }

    pub fn process_audio(&mut self, samples: &[f32]) -> &[f32] {
        let size = self.buffer.len();

        if samples.len() < size {
            return &[];
        }

        for (i, &sample) in samples.iter().take(size).enumerate() {
            self.buffer[i] = Complex::new(sample * self.window[i], 0.0);
        }

        self.fft.process_with_scratch(&mut self.buffer, &mut self.scratch);

        for (i, complex) in self.buffer.iter().take(size / 2).enumerate() {
            self.output_buffer[i] = complex.norm();
        }

        &self.output_buffer
    }

    fn hann_window(size: usize) -> Vec<f32> {
        (0..size)
            .map(|i| {
                let phase = 2.0 * std::f32::consts::PI * i as f32 / (size - 1) as f32;
                0.5 * (1.0 - phase.cos())
            })
            .collect()
    }

    pub fn get_frequency_bin(&self, bin: usize, sample_rate: f32) -> f32 {
        bin as f32 * sample_rate / (2.0 * self.buffer.len() as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_fft_processing() {
        let mut analyzer = FftAnalyzer::new(1024);

        let sample_rate = 44100.0;
        let frequency = 1000.0;
        let samples: Vec<f32> = (0..1024)
            .map(|i| {
                let t = i as f32 / sample_rate;
                (2.0 * std::f32::consts::PI * frequency * t).sin()
            })
            .collect();

        let result = analyzer.process_audio(&samples);

        assert!(result.len() > 0);

        let expected_bin = (frequency / sample_rate * 1024.0) as usize;
        let peak_bin = result
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        assert_abs_diff_eq!(peak_bin as f32, expected_bin as f32, epsilon = 2.0);
    }

    #[test]
    fn test_hann_window() {
        let window = FftAnalyzer::hann_window(8);
        assert_abs_diff_eq!(window[0], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(window[7], 0.0, epsilon = 1e-6);
        assert!(window[4] > 0.9);
    }
}