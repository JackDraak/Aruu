use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SmoothingType {
    Linear(f32),      // factor: 0.0 = no smoothing, 1.0 = instant change
    Exponential(f32), // decay: higher = faster response
    Adaptive { min_factor: f32, max_factor: f32, sensitivity: f32 },
}

impl SmoothingType {
    pub fn linear(factor: f32) -> Self {
        Self::Linear(factor.clamp(0.0, 1.0))
    }

    pub fn exponential(decay: f32) -> Self {
        Self::Exponential(decay.max(0.1))
    }

    pub fn adaptive(min_factor: f32, max_factor: f32, sensitivity: f32) -> Self {
        Self::Adaptive {
            min_factor: min_factor.clamp(0.0, 1.0),
            max_factor: max_factor.clamp(0.0, 1.0),
            sensitivity: sensitivity.max(0.1),
        }
    }
}

pub struct Smoother {
    smoothing_configs: HashMap<String, SmoothingType>,
    previous_values: HashMap<String, f32>,
    change_rates: HashMap<String, f32>,
}

impl Smoother {
    pub fn new() -> Self {
        Self {
            smoothing_configs: HashMap::new(),
            previous_values: HashMap::new(),
            change_rates: HashMap::new(),
        }
    }

    pub fn configure(&mut self, param_name: &str, smoothing_type: SmoothingType) {
        self.smoothing_configs.insert(param_name.to_string(), smoothing_type);
    }

    pub fn configure_multiple(&mut self, configs: &[(&str, SmoothingType)]) {
        for (name, smoothing_type) in configs {
            self.configure(name, smoothing_type.clone());
        }
    }

    pub fn smooth(&mut self, param_name: &str, new_value: f32) -> f32 {
        let previous = self.previous_values.get(param_name).copied().unwrap_or(new_value);

        let smoothed_value = if let Some(smoothing_type) = self.smoothing_configs.get(param_name) {
            self.apply_smoothing(smoothing_type, previous, new_value, param_name)
        } else {
            new_value
        };

        let change_rate = (new_value - previous).abs();
        self.change_rates.insert(param_name.to_string(), change_rate);
        self.previous_values.insert(param_name.to_string(), smoothed_value);

        smoothed_value
    }

    fn apply_smoothing(&self, smoothing_type: &SmoothingType, previous: f32, new_value: f32, param_name: &str) -> f32 {
        match smoothing_type {
            SmoothingType::Linear(factor) => {
                lerp(previous, new_value, *factor)
            }
            SmoothingType::Exponential(decay) => {
                let dt = 1.0 / 60.0;
                let alpha = 1.0 - (-decay * dt).exp();
                lerp(previous, new_value, alpha)
            }
            SmoothingType::Adaptive { min_factor, max_factor, sensitivity } => {
                let change_rate = self.change_rates.get(param_name).copied().unwrap_or(0.0);
                let normalized_change = (change_rate * sensitivity).min(1.0);
                let adaptive_factor = lerp(*min_factor, *max_factor, normalized_change);
                lerp(previous, new_value, adaptive_factor)
            }
        }
    }

    pub fn smooth_multiple<'a>(&mut self, values: &[(&'a str, f32)]) -> Vec<(&'a str, f32)> {
        values.iter()
            .map(|(name, value)| (*name, self.smooth(name, *value)))
            .collect()
    }

    pub fn get_change_rate(&self, param_name: &str) -> f32 {
        self.change_rates.get(param_name).copied().unwrap_or(0.0)
    }

    pub fn reset(&mut self, param_name: &str) {
        self.previous_values.remove(param_name);
        self.change_rates.remove(param_name);
    }

    pub fn reset_all(&mut self) {
        self.previous_values.clear();
        self.change_rates.clear();
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub trait Smoothable {
    fn apply_smoothing(&mut self, smoother: &mut Smoother);
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_linear_smoothing() {
        let mut smoother = Smoother::new();
        smoother.configure("test", SmoothingType::linear(0.5));

        let result1 = smoother.smooth("test", 1.0);
        assert_abs_diff_eq!(result1, 1.0, epsilon = 0.001);

        let result2 = smoother.smooth("test", 0.0);
        assert_abs_diff_eq!(result2, 0.5, epsilon = 0.001);
    }

    #[test]
    fn test_exponential_smoothing() {
        let mut smoother = Smoother::new();
        smoother.configure("test", SmoothingType::exponential(5.0));

        let result1 = smoother.smooth("test", 1.0);
        assert_abs_diff_eq!(result1, 1.0, epsilon = 0.001);

        let result2 = smoother.smooth("test", 0.0);
        assert!(result2 > 0.0 && result2 < 1.0);
    }

    #[test]
    fn test_adaptive_smoothing() {
        let mut smoother = Smoother::new();
        smoother.configure("test", SmoothingType::adaptive(0.1, 0.8, 2.0));

        smoother.smooth("test", 0.0);
        let small_change = smoother.smooth("test", 0.1);

        smoother.smooth("test", 0.0);
        let large_change = smoother.smooth("test", 1.0);

        assert!(large_change != small_change);
    }

    #[test]
    fn test_multiple_smoothing() {
        let mut smoother = Smoother::new();
        smoother.configure_multiple(&[
            ("param1", SmoothingType::linear(0.3)),
            ("param2", SmoothingType::linear(0.7)),
        ]);

        let results = smoother.smooth_multiple(&[
            ("param1", 1.0),
            ("param2", 1.0),
        ]);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "param1");
        assert_eq!(results[1].0, "param2");
    }
}