use anyhow::Result;
use std::time::{Duration, Instant};

/// Performance quality levels for adaptive rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityLevel {
    /// Maximum quality with full effects
    Ultra,
    /// High quality with some optimizations
    High,
    /// Balanced quality and performance
    Medium,
    /// Performance-focused with simplified effects
    Low,
    /// Minimal quality for very low-end hardware
    Potato,
}

impl QualityLevel {
    /// Get the resolution scale factor for this quality level
    pub fn resolution_scale(&self) -> f32 {
        match self {
            QualityLevel::Ultra => 1.0,
            QualityLevel::High => 1.0,
            QualityLevel::Medium => 0.8,
            QualityLevel::Low => 0.6,
            QualityLevel::Potato => 0.5,
        }
    }

    /// Get the pattern complexity multiplier
    pub fn complexity_multiplier(&self) -> f32 {
        match self {
            QualityLevel::Ultra => 1.0,
            QualityLevel::High => 0.9,
            QualityLevel::Medium => 0.7,
            QualityLevel::Low => 0.5,
            QualityLevel::Potato => 0.3,
        }
    }

    /// Get maximum shader iterations for this quality level
    pub fn max_iterations(&self) -> u32 {
        match self {
            QualityLevel::Ultra => 128,
            QualityLevel::High => 96,
            QualityLevel::Medium => 64,
            QualityLevel::Low => 32,
            QualityLevel::Potato => 16,
        }
    }

    /// Get effect intensity scaling
    pub fn effect_intensity(&self) -> f32 {
        match self {
            QualityLevel::Ultra => 1.0,
            QualityLevel::High => 0.95,
            QualityLevel::Medium => 0.8,
            QualityLevel::Low => 0.6,
            QualityLevel::Potato => 0.4,
        }
    }

    /// Check if advanced effects should be enabled
    pub fn enable_advanced_effects(&self) -> bool {
        matches!(self, QualityLevel::Ultra | QualityLevel::High)
    }

    /// Check if particle systems should be enabled
    pub fn enable_particles(&self) -> bool {
        !matches!(self, QualityLevel::Potato)
    }

    /// Get noise octaves for procedural generation
    pub fn noise_octaves(&self) -> u32 {
        match self {
            QualityLevel::Ultra => 6,
            QualityLevel::High => 5,
            QualityLevel::Medium => 4,
            QualityLevel::Low => 3,
            QualityLevel::Potato => 2,
        }
    }
}

/// Performance metrics tracking
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub frame_time: Duration,
    pub cpu_time: Duration,
    pub gpu_time: Duration,
    pub fps: f32,
    pub dropped_frames: u32,
    pub memory_usage_mb: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_time: Duration::from_millis(16), // Target 60 FPS
            cpu_time: Duration::from_millis(5),
            gpu_time: Duration::from_millis(11),
            fps: 60.0,
            dropped_frames: 0,
            memory_usage_mb: 100.0,
        }
    }
}

/// Adaptive performance manager
pub struct PerformanceManager {
    current_quality: QualityLevel,
    target_fps: f32,
    metrics_history: Vec<PerformanceMetrics>,
    last_adjustment: Instant,
    adjustment_cooldown: Duration,
    consecutive_poor_frames: u32,
    consecutive_good_frames: u32,
}

impl PerformanceManager {
    pub fn new(target_fps: f32) -> Self {
        Self {
            current_quality: QualityLevel::High, // Start optimistic
            target_fps,
            metrics_history: Vec::with_capacity(60), // Store 1 second of history
            last_adjustment: Instant::now(),
            adjustment_cooldown: Duration::from_secs(2), // Don't adjust too frequently
            consecutive_poor_frames: 0,
            consecutive_good_frames: 0,
        }
    }

    /// Update performance metrics and potentially adjust quality
    pub fn update(&mut self, metrics: PerformanceMetrics) -> bool {
        let mut quality_changed = false;

        // Add to history (keep only recent samples)
        self.metrics_history.push(metrics.clone());
        if self.metrics_history.len() > 60 {
            self.metrics_history.remove(0);
        }

        // Check if we should consider adjusting quality
        if self.last_adjustment.elapsed() >= self.adjustment_cooldown {
            let target_frame_time = Duration::from_secs_f32(1.0 / self.target_fps);
            let performance_ratio = metrics.frame_time.as_secs_f32() / target_frame_time.as_secs_f32();

            if performance_ratio > 1.2 {
                // Frame time is 20% over target
                self.consecutive_poor_frames += 1;
                self.consecutive_good_frames = 0;

                if self.consecutive_poor_frames >= 5 {
                    quality_changed = self.decrease_quality();
                }
            } else if performance_ratio < 0.8 {
                // Frame time is 20% under target - we have headroom
                self.consecutive_good_frames += 1;
                self.consecutive_poor_frames = 0;

                if self.consecutive_good_frames >= 15 {
                    quality_changed = self.increase_quality();
                }
            } else {
                // Performance is acceptable
                self.consecutive_poor_frames = 0;
                self.consecutive_good_frames = 0;
            }
        }

        quality_changed
    }

    /// Decrease quality level to improve performance
    fn decrease_quality(&mut self) -> bool {
        let old_quality = self.current_quality;

        self.current_quality = match self.current_quality {
            QualityLevel::Ultra => QualityLevel::High,
            QualityLevel::High => QualityLevel::Medium,
            QualityLevel::Medium => QualityLevel::Low,
            QualityLevel::Low => QualityLevel::Potato,
            QualityLevel::Potato => QualityLevel::Potato, // Already at minimum
        };

        if self.current_quality != old_quality {
            println!("🔻 Performance: Decreased quality to {:?}", self.current_quality);
            self.last_adjustment = Instant::now();
            self.consecutive_poor_frames = 0;
            true
        } else {
            false
        }
    }

    /// Increase quality level when performance allows
    fn increase_quality(&mut self) -> bool {
        let old_quality = self.current_quality;

        self.current_quality = match self.current_quality {
            QualityLevel::Potato => QualityLevel::Low,
            QualityLevel::Low => QualityLevel::Medium,
            QualityLevel::Medium => QualityLevel::High,
            QualityLevel::High => QualityLevel::Ultra,
            QualityLevel::Ultra => QualityLevel::Ultra, // Already at maximum
        };

        if self.current_quality != old_quality {
            println!("🔺 Performance: Increased quality to {:?}", self.current_quality);
            self.last_adjustment = Instant::now();
            self.consecutive_good_frames = 0;
            true
        } else {
            false
        }
    }

    /// Get current quality level
    pub fn current_quality(&self) -> QualityLevel {
        self.current_quality
    }

    /// Force set quality level (for user override)
    pub fn set_quality(&mut self, quality: QualityLevel) {
        if self.current_quality != quality {
            println!("🎛️  Performance: Quality manually set to {:?}", quality);
            self.current_quality = quality;
            self.last_adjustment = Instant::now();
            self.consecutive_poor_frames = 0;
            self.consecutive_good_frames = 0;
        }
    }

    /// Get average FPS over recent history
    pub fn average_fps(&self) -> f32 {
        if self.metrics_history.is_empty() {
            return 60.0;
        }

        let sum: f32 = self.metrics_history.iter().map(|m| m.fps).sum();
        sum / self.metrics_history.len() as f32
    }

    /// Get 99th percentile frame time (worst 1% of frames)
    pub fn percentile_99_frame_time(&self) -> Duration {
        if self.metrics_history.is_empty() {
            return Duration::from_millis(16);
        }

        let mut frame_times: Vec<Duration> = self.metrics_history
            .iter()
            .map(|m| m.frame_time)
            .collect();

        frame_times.sort();

        let index = (frame_times.len() as f32 * 0.99) as usize;
        frame_times.get(index.min(frame_times.len() - 1))
            .copied()
            .unwrap_or(Duration::from_millis(16))
    }

    /// Get performance report for debugging
    pub fn performance_report(&self) -> String {
        format!(
            "Quality: {:?} | Avg FPS: {:.1} | P99 Frame Time: {:.1}ms | History: {} samples",
            self.current_quality,
            self.average_fps(),
            self.percentile_99_frame_time().as_secs_f32() * 1000.0,
            self.metrics_history.len()
        )
    }
}

/// GPU capability detection and shader compatibility
pub struct GpuCapabilities {
    pub max_texture_size: u32,
    pub max_compute_workgroups: u32,
    pub supports_compute_shaders: bool,
    pub memory_gb: f32,
    pub recommended_quality: QualityLevel,
}

impl GpuCapabilities {
    /// Detect GPU capabilities from WGPU limits
    pub fn detect(limits: &wgpu::Limits) -> Self {
        let max_texture_size = limits.max_texture_dimension_2d;
        let max_compute_workgroups = limits.max_compute_workgroups_per_dimension;

        // Estimate capability based on limits
        let recommended_quality = if max_texture_size >= 8192 {
            QualityLevel::Ultra
        } else if max_texture_size >= 4096 {
            QualityLevel::High
        } else if max_texture_size >= 2048 {
            QualityLevel::Medium
        } else if max_texture_size >= 1024 {
            QualityLevel::Low
        } else {
            QualityLevel::Potato
        };

        Self {
            max_texture_size,
            max_compute_workgroups,
            supports_compute_shaders: max_compute_workgroups > 0,
            memory_gb: 2.0, // Conservative estimate
            recommended_quality,
        }
    }

    /// Check if a shader type is supported at current quality
    pub fn supports_shader(&self, shader_cost: u32, quality: QualityLevel) -> bool {
        let max_cost = match quality {
            QualityLevel::Ultra => 10,
            QualityLevel::High => 8,
            QualityLevel::Medium => 6,
            QualityLevel::Low => 4,
            QualityLevel::Potato => 2,
        };

        shader_cost <= max_cost
    }
}

/// Performance-aware shader parameters
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PerformanceUniforms {
    pub quality_level: f32,           // 0.0-4.0 (Potato to Ultra)
    pub complexity_multiplier: f32,   // Pattern complexity scaling
    pub max_iterations: f32,          // Maximum shader iterations
    pub effect_intensity: f32,        // Global effect intensity
    pub resolution_scale: f32,        // Render resolution scaling
    pub enable_advanced_effects: f32, // 1.0 = enabled, 0.0 = disabled
    pub enable_particles: f32,        // 1.0 = enabled, 0.0 = disabled
    pub noise_octaves: f32,           // Number of noise octaves
}

impl From<QualityLevel> for PerformanceUniforms {
    fn from(quality: QualityLevel) -> Self {
        Self {
            quality_level: match quality {
                QualityLevel::Potato => 0.0,
                QualityLevel::Low => 1.0,
                QualityLevel::Medium => 2.0,
                QualityLevel::High => 3.0,
                QualityLevel::Ultra => 4.0,
            },
            complexity_multiplier: quality.complexity_multiplier(),
            max_iterations: quality.max_iterations() as f32,
            effect_intensity: quality.effect_intensity(),
            resolution_scale: quality.resolution_scale(),
            enable_advanced_effects: if quality.enable_advanced_effects() { 1.0 } else { 0.0 },
            enable_particles: if quality.enable_particles() { 1.0 } else { 0.0 },
            noise_octaves: quality.noise_octaves() as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_level_properties() {
        assert_eq!(QualityLevel::Ultra.resolution_scale(), 1.0);
        assert_eq!(QualityLevel::Potato.resolution_scale(), 0.5);
        assert!(QualityLevel::Ultra.enable_advanced_effects());
        assert!(!QualityLevel::Potato.enable_advanced_effects());
    }

    #[test]
    fn test_performance_manager_creation() {
        let manager = PerformanceManager::new(60.0);
        assert_eq!(manager.current_quality(), QualityLevel::High);
        assert_eq!(manager.average_fps(), 60.0); // Default when no history
    }

    #[test]
    fn test_performance_adjustment() {
        let mut manager = PerformanceManager::new(60.0);

        // Simulate poor performance
        let poor_metrics = PerformanceMetrics {
            frame_time: Duration::from_millis(25), // 40 FPS
            fps: 40.0,
            ..Default::default()
        };

        // Should not adjust immediately due to cooldown
        assert!(!manager.update(poor_metrics.clone()));

        // Force cooldown to pass
        manager.last_adjustment = Instant::now() - Duration::from_secs(3);

        // Feed several poor frames
        for _ in 0..6 {
            manager.update(poor_metrics.clone());
        }

        // Quality should have been reduced
        assert_ne!(manager.current_quality(), QualityLevel::High);
    }

    #[test]
    fn test_gpu_capabilities_detection() {
        let limits = wgpu::Limits {
            max_texture_dimension_2d: 4096,
            max_compute_workgroups_per_dimension: 256,
            ..Default::default()
        };

        let capabilities = GpuCapabilities::detect(&limits);
        assert_eq!(capabilities.recommended_quality, QualityLevel::High);
        assert!(capabilities.supports_shader(6, QualityLevel::High));
        assert!(!capabilities.supports_shader(10, QualityLevel::Medium));
    }

    #[test]
    fn test_performance_uniforms_conversion() {
        let uniforms = PerformanceUniforms::from(QualityLevel::Medium);
        assert_eq!(uniforms.quality_level, 2.0);
        assert_eq!(uniforms.resolution_scale, 0.8);
        assert_eq!(uniforms.noise_octaves, 4.0);
    }
}