/// Epilepsy Safety Engine for Aruu Audio Visualizer
///
/// Implements photosensitive epilepsy prevention measures based on international standards:
/// - WCAG 2.0 Guidelines: ≤3 flashes per second, ≤10% luminance change
/// - Gaming Industry Standards: Xbox, PlayStation, Steam safety requirements
/// - Medical Research: 5-30 Hz range most dangerous, particularly 15-20 Hz
///
/// Core Safety Principles:
/// - "Maximum Audio Response, Minimum Seizure Risk"
/// - Preserve musical reactivity while ensuring user safety
/// - Intelligent dampening rather than blanket restrictions

use std::time::Instant;

/// Simple 3D vector for RGB color operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Vector3<f32> {
    /// Multiply vector by scalar
    pub fn mul_scalar(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Mul<f32> for Vector3<f32> {
    type Output = Vector3<f32>;

    fn mul(self, scalar: f32) -> Self::Output {
        self.mul_scalar(scalar)
    }
}

/// Core safety limits based on international standards
pub const FLASH_RATE_LIMIT_HZ: f32 = 3.0;  // Maximum 3 flashes per second
pub const LUMINANCE_CHANGE_LIMIT: f32 = 0.1; // Maximum 10% brightness change
pub const RED_FLASH_LIMIT_HZ: f32 = 3.0;     // Red flashes most dangerous
pub const SAFETY_COOLDOWN_SECONDS: f32 = 1.0 / FLASH_RATE_LIMIT_HZ; // 333ms between major changes

/// Safety levels for user control
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SafetyLevel {
    /// Ultra-conservative for maximum safety
    UltraSafe,
    /// Conservative for users with mild sensitivity
    Safe,
    /// Moderate for general use
    Moderate,
    /// Standard for users without known sensitivity
    Standard,
    /// Disabled (only for medical professionals/testing)
    Disabled,
}

impl Default for SafetyLevel {
    fn default() -> Self {
        SafetyLevel::Safe // Default to conservative settings
    }
}

/// Tracks visual changes to prevent dangerous flash patterns
#[derive(Debug)]
pub struct FlashTracker {
    last_major_change: Instant,
    last_red_flash: Instant,
    recent_changes: Vec<(Instant, f32)>, // (time, intensity) pairs
    change_accumulator: f32,
}

impl FlashTracker {
    pub fn new() -> Self {
        // Initialize with past timestamps to allow first flash
        let past_time = Instant::now() - std::time::Duration::from_secs(1);
        Self {
            last_major_change: past_time,
            last_red_flash: past_time,
            recent_changes: Vec::new(),
            change_accumulator: 0.0,
        }
    }

    /// Check if a visual change is safe to allow
    pub fn can_allow_change(&mut self, intensity: f32, is_red_dominant: bool) -> bool {
        let now = Instant::now();

        // Clean old changes (only keep last second)
        self.recent_changes.retain(|(time, _)| now.duration_since(*time).as_secs_f32() < 1.0);

        // Check red flash specific limits (most dangerous)
        if is_red_dominant && intensity > 0.3 {
            let time_since_red = now.duration_since(self.last_red_flash).as_secs_f32();
            if time_since_red < SAFETY_COOLDOWN_SECONDS {
                return false;
            }
        }

        // Check general flash rate
        if intensity > 0.3 {
            let time_since_major = now.duration_since(self.last_major_change).as_secs_f32();
            if time_since_major < SAFETY_COOLDOWN_SECONDS {
                return false;
            }

            // Count recent major changes
            let recent_major_changes = self.recent_changes.iter()
                .filter(|(_, i)| *i > 0.3)
                .count();

            if recent_major_changes >= 3 {
                return false; // Already at 3 Hz limit
            }
        }

        true
    }

    /// Record a visual change for tracking
    pub fn record_change(&mut self, intensity: f32, is_red_dominant: bool) {
        let now = Instant::now();

        if intensity > 0.3 {
            self.last_major_change = now;

            if is_red_dominant {
                self.last_red_flash = now;
            }
        }

        self.recent_changes.push((now, intensity));
        self.change_accumulator += intensity;
    }
}

/// Controls luminance changes to prevent dangerous brightness variations
#[derive(Debug)]
pub struct LuminanceLimiter {
    previous_luminance: f32,
    luminance_history: Vec<(Instant, f32)>,
}

impl LuminanceLimiter {
    pub fn new() -> Self {
        Self {
            previous_luminance: 0.5, // Start with medium brightness
            luminance_history: Vec::new(),
        }
    }

    /// Calculate relative luminance from RGB values (ITU-R BT.709 standard)
    pub fn calculate_luminance(rgb: Vector3<f32>) -> f32 {
        0.2126 * rgb.x + 0.7152 * rgb.y + 0.0722 * rgb.z
    }

    /// Limit luminance change to safe levels
    pub fn limit_luminance_change(&mut self, new_rgb: Vector3<f32>) -> Vector3<f32> {
        let new_luminance = Self::calculate_luminance(new_rgb);
        let luminance_delta = (new_luminance - self.previous_luminance).abs();

        if luminance_delta > LUMINANCE_CHANGE_LIMIT {
            // Interpolate to safe luminance level
            let safe_luminance = if new_luminance > self.previous_luminance {
                self.previous_luminance + LUMINANCE_CHANGE_LIMIT
            } else {
                self.previous_luminance - LUMINANCE_CHANGE_LIMIT
            };

            // Scale RGB to achieve safe luminance
            let luminance_ratio = safe_luminance / new_luminance.max(0.001);
            let safe_rgb = new_rgb * luminance_ratio;

            self.previous_luminance = safe_luminance;

            // Record this change
            self.luminance_history.push((Instant::now(), safe_luminance));

            safe_rgb
        } else {
            self.previous_luminance = new_luminance;
            self.luminance_history.push((Instant::now(), new_luminance));
            new_rgb
        }
    }

    /// Get recent luminance change rate for monitoring
    pub fn get_change_rate(&self) -> f32 {
        if self.luminance_history.len() < 2 {
            return 0.0;
        }

        let now = Instant::now();
        let recent_changes: Vec<_> = self.luminance_history.iter()
            .filter(|(time, _)| now.duration_since(*time).as_secs_f32() < 1.0)
            .collect();

        if recent_changes.len() < 2 {
            return 0.0;
        }

        let mut total_change = 0.0;
        for i in 1..recent_changes.len() {
            total_change += (recent_changes[i].1 - recent_changes[i-1].1).abs();
        }

        total_change / recent_changes.len() as f32
    }
}

/// Main Safety Engine coordinating all safety systems
pub struct SafetyEngine {
    flash_tracker: FlashTracker,
    luminance_limiter: LuminanceLimiter,
    safety_level: SafetyLevel,
    emergency_stop: bool,
    safety_warnings: Vec<String>,
}

impl SafetyEngine {
    pub fn new() -> Self {
        Self {
            flash_tracker: FlashTracker::new(),
            luminance_limiter: LuminanceLimiter::new(),
            safety_level: SafetyLevel::default(),
            emergency_stop: false,
            safety_warnings: Vec::new(),
        }
    }

    /// Configure safety level
    pub fn set_safety_level(&mut self, level: SafetyLevel) {
        self.safety_level = level;
    }

    /// Get current safety level
    pub fn get_safety_level(&self) -> SafetyLevel {
        self.safety_level
    }

    /// Emergency stop all visual effects
    pub fn emergency_stop(&mut self) {
        self.emergency_stop = true;
        self.safety_warnings.push("Emergency stop activated".to_string());
    }

    /// Resume from emergency stop
    pub fn resume(&mut self) {
        self.emergency_stop = false;
        self.safety_warnings.clear();
    }

    /// Check if emergency stop is active
    pub fn is_emergency_stopped(&self) -> bool {
        self.emergency_stop
    }

    /// Apply safety filtering to color values
    pub fn filter_color(&mut self, color: Vector3<f32>) -> Vector3<f32> {
        if self.emergency_stop {
            return Vector3::new(0.1, 0.1, 0.1); // Very dim gray in emergency
        }

        // Apply safety level modifications
        let intensity_limit = match self.safety_level {
            SafetyLevel::UltraSafe => 0.3,
            SafetyLevel::Safe => 0.5,
            SafetyLevel::Moderate => 0.7,
            SafetyLevel::Standard => 0.9,
            SafetyLevel::Disabled => 1.0,
        };

        // Clamp color intensity
        let limited_color = color * intensity_limit;

        // Apply luminance limiting
        self.luminance_limiter.limit_luminance_change(limited_color)
    }

    /// Check if a visual effect is safe to display
    pub fn can_allow_effect(&mut self, intensity: f32, color: Vector3<f32>) -> bool {
        if self.emergency_stop {
            return false;
        }

        if self.safety_level == SafetyLevel::Disabled {
            return true; // No restrictions when disabled
        }

        // Check for red dominance (most dangerous wavelength)
        let is_red_dominant = color.x > color.y * 1.5 && color.x > color.z * 1.5;

        // Apply stricter limits for higher safety levels
        let adjusted_intensity = match self.safety_level {
            SafetyLevel::UltraSafe => intensity * 0.3,
            SafetyLevel::Safe => intensity * 0.5,
            SafetyLevel::Moderate => intensity * 0.7,
            SafetyLevel::Standard => intensity * 0.9,
            SafetyLevel::Disabled => intensity,
        };

        self.flash_tracker.can_allow_change(adjusted_intensity, is_red_dominant)
    }

    /// Record a visual effect for safety tracking
    pub fn record_effect(&mut self, intensity: f32, color: Vector3<f32>) {
        if self.safety_level == SafetyLevel::Disabled {
            return;
        }

        let is_red_dominant = color.x > color.y * 1.5 && color.x > color.z * 1.5;
        self.flash_tracker.record_change(intensity, is_red_dominant);
    }

    /// Get current safety status for monitoring
    pub fn get_safety_status(&self) -> SafetyStatus {
        SafetyStatus {
            level: self.safety_level,
            emergency_stopped: self.emergency_stop,
            luminance_change_rate: self.luminance_limiter.get_change_rate(),
            warnings: self.safety_warnings.clone(),
        }
    }

    /// Get safety multipliers for audio-reactive effects
    pub fn get_safety_multipliers(&self) -> SafetyMultipliers {
        if self.emergency_stop {
            return SafetyMultipliers::emergency_stop();
        }

        match self.safety_level {
            SafetyLevel::UltraSafe => SafetyMultipliers::ultra_safe(),
            SafetyLevel::Safe => SafetyMultipliers::safe(),
            SafetyLevel::Moderate => SafetyMultipliers::moderate(),
            SafetyLevel::Standard => SafetyMultipliers::standard(),
            SafetyLevel::Disabled => SafetyMultipliers::disabled(),
        }
    }
}

/// Safety multipliers for audio-reactive effects
#[derive(Debug, Clone, Copy)]
pub struct SafetyMultipliers {
    pub beat_intensity: f32,
    pub onset_intensity: f32,
    pub color_change_rate: f32,
    pub brightness_range: f32,
    pub pattern_complexity: f32,
}

impl SafetyMultipliers {
    pub fn emergency_stop() -> Self {
        Self {
            beat_intensity: 0.0,
            onset_intensity: 0.0,
            color_change_rate: 0.0,
            brightness_range: 0.1,
            pattern_complexity: 0.0,
        }
    }

    pub fn ultra_safe() -> Self {
        Self {
            beat_intensity: 0.1,
            onset_intensity: 0.05,
            color_change_rate: 0.2,
            brightness_range: 0.3,
            pattern_complexity: 0.3,
        }
    }

    pub fn safe() -> Self {
        Self {
            beat_intensity: 0.3,
            onset_intensity: 0.2,
            color_change_rate: 0.4,
            brightness_range: 0.5,
            pattern_complexity: 0.5,
        }
    }

    pub fn moderate() -> Self {
        Self {
            beat_intensity: 0.6,
            onset_intensity: 0.4,
            color_change_rate: 0.7,
            brightness_range: 0.7,
            pattern_complexity: 0.7,
        }
    }

    pub fn standard() -> Self {
        Self {
            beat_intensity: 0.8,
            onset_intensity: 0.6,
            color_change_rate: 0.9,
            brightness_range: 0.9,
            pattern_complexity: 0.9,
        }
    }

    pub fn disabled() -> Self {
        Self {
            beat_intensity: 1.0,
            onset_intensity: 1.0,
            color_change_rate: 1.0,
            brightness_range: 1.0,
            pattern_complexity: 1.0,
        }
    }
}

/// Current safety status for monitoring and display
#[derive(Debug, Clone)]
pub struct SafetyStatus {
    pub level: SafetyLevel,
    pub emergency_stopped: bool,
    pub luminance_change_rate: f32,
    pub warnings: Vec<String>,
}

impl SafetyStatus {
    /// Get a human-readable status message
    pub fn get_status_message(&self) -> String {
        if self.emergency_stopped {
            return "⛔ EMERGENCY STOP ACTIVE".to_string();
        }

        let level_str = match self.level {
            SafetyLevel::UltraSafe => "🛡️ Ultra Safe",
            SafetyLevel::Safe => "🔒 Safe",
            SafetyLevel::Moderate => "⚠️ Moderate",
            SafetyLevel::Standard => "🎨 Standard",
            SafetyLevel::Disabled => "⚠️ DISABLED",
        };

        let change_indicator = if self.luminance_change_rate > 0.05 {
            " (High Activity)"
        } else {
            ""
        };

        format!("{}{}", level_str, change_indicator)
    }

    /// Check if user should be warned about current activity level
    pub fn should_warn_user(&self) -> bool {
        !self.warnings.is_empty() || self.luminance_change_rate > 0.08
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_rate_limiting() {
        let mut tracker = FlashTracker::new();

        // First change should be allowed
        assert!(tracker.can_allow_change(0.5, false));
        tracker.record_change(0.5, false);

        // Immediate second change should be blocked
        assert!(!tracker.can_allow_change(0.5, false));
    }

    #[test]
    fn test_luminance_limiting() {
        let mut limiter = LuminanceLimiter::new();

        let bright_color = Vector3::new(1.0, 1.0, 1.0);
        let limited = limiter.limit_luminance_change(bright_color);

        // Should be limited from previous luminance (0.5)
        assert!(LuminanceLimiter::calculate_luminance(limited) <= 0.6);
    }

    #[test]
    fn test_safety_multipliers() {
        let ultra_safe = SafetyMultipliers::ultra_safe();
        let standard = SafetyMultipliers::standard();

        // Ultra safe should be much more conservative
        assert!(ultra_safe.beat_intensity < standard.beat_intensity);
        assert!(ultra_safe.onset_intensity < standard.onset_intensity);
    }

    #[test]
    fn test_emergency_stop() {
        let mut engine = SafetyEngine::new();

        engine.emergency_stop();
        assert!(engine.is_emergency_stopped());

        let safe_color = engine.filter_color(Vector3::new(1.0, 0.0, 0.0));
        assert!(safe_color.x < 0.2); // Should be very dim in emergency
    }

    // ===== NEW COMPREHENSIVE SAFETY PIPELINE TESTS =====

    #[test]
    fn test_safety_multiplier_progression() {
        let ultra_safe = SafetyMultipliers::ultra_safe();
        let safe = SafetyMultipliers::safe();
        let moderate = SafetyMultipliers::moderate();
        let standard = SafetyMultipliers::standard();
        let disabled = SafetyMultipliers::disabled();

        // Validate progression: UltraSafe < Safe < Moderate < Standard < Disabled
        assert!(ultra_safe.beat_intensity < safe.beat_intensity);
        assert!(safe.beat_intensity < moderate.beat_intensity);
        assert!(moderate.beat_intensity < standard.beat_intensity);
        assert!(standard.beat_intensity < disabled.beat_intensity);

        // Same for onset intensity
        assert!(ultra_safe.onset_intensity < safe.onset_intensity);
        assert!(safe.onset_intensity < moderate.onset_intensity);
        assert!(moderate.onset_intensity < standard.onset_intensity);
        assert!(standard.onset_intensity < disabled.onset_intensity);

        // Validate disabled is full intensity (1.0)
        assert_eq!(disabled.beat_intensity, 1.0);
        assert_eq!(disabled.onset_intensity, 1.0);
        assert_eq!(disabled.brightness_range, 1.0);

        // Validate emergency stop is minimal
        let emergency = SafetyMultipliers::emergency_stop();
        assert_eq!(emergency.beat_intensity, 0.0);
        assert_eq!(emergency.onset_intensity, 0.0);
        assert_eq!(emergency.brightness_range, 0.1);
    }

    #[test]
    fn test_safety_engine_level_transitions() {
        let mut engine = SafetyEngine::new();

        // Test each safety level produces correct multipliers
        engine.set_safety_level(SafetyLevel::UltraSafe);
        let ultra_multipliers = engine.get_safety_multipliers();
        assert_eq!(ultra_multipliers.beat_intensity, 0.1);
        assert_eq!(ultra_multipliers.onset_intensity, 0.05);

        engine.set_safety_level(SafetyLevel::Safe);
        let safe_multipliers = engine.get_safety_multipliers();
        assert_eq!(safe_multipliers.beat_intensity, 0.3);
        assert_eq!(safe_multipliers.onset_intensity, 0.2);

        engine.set_safety_level(SafetyLevel::Standard);
        let standard_multipliers = engine.get_safety_multipliers();
        assert_eq!(standard_multipliers.beat_intensity, 0.8);
        assert_eq!(standard_multipliers.onset_intensity, 0.6);

        engine.set_safety_level(SafetyLevel::Disabled);
        let disabled_multipliers = engine.get_safety_multipliers();
        assert_eq!(disabled_multipliers.beat_intensity, 1.0);
        assert_eq!(disabled_multipliers.onset_intensity, 1.0);
    }

    #[test]
    fn test_emergency_stop_overrides_all_levels() {
        let mut engine = SafetyEngine::new();

        // Test emergency stop overrides all safety levels
        let levels = [
            SafetyLevel::UltraSafe,
            SafetyLevel::Safe,
            SafetyLevel::Moderate,
            SafetyLevel::Standard,
            SafetyLevel::Disabled,
        ];

        for level in levels {
            engine.set_safety_level(level);
            engine.emergency_stop();

            let multipliers = engine.get_safety_multipliers();
            assert_eq!(multipliers.beat_intensity, 0.0, "Emergency stop should override {:?}", level);
            assert_eq!(multipliers.onset_intensity, 0.0, "Emergency stop should override {:?}", level);
            assert_eq!(multipliers.brightness_range, 0.1, "Emergency stop should override {:?}", level);

            engine.resume();
        }
    }

    #[test]
    fn test_safety_status_reporting() {
        let mut engine = SafetyEngine::new();

        // Test normal status reporting
        let status = engine.get_safety_status();
        assert_eq!(status.level, SafetyLevel::Safe); // Default level
        assert!(!status.emergency_stopped);
        assert!(status.warnings.is_empty());

        // Test emergency stop status
        engine.emergency_stop();
        let emergency_status = engine.get_safety_status();
        assert!(emergency_status.emergency_stopped);
        assert!(!emergency_status.warnings.is_empty());

        // Test status message
        assert!(emergency_status.get_status_message().contains("EMERGENCY STOP"));

        // Test resume clears warnings
        engine.resume();
        let resumed_status = engine.get_safety_status();
        assert!(!resumed_status.emergency_stopped);
        assert!(resumed_status.warnings.is_empty());
    }

    #[test]
    fn test_red_flash_detection_and_limiting() {
        let mut engine = SafetyEngine::new();

        // Test red-dominant color detection and stricter limiting
        let red_color = Vector3::new(1.0, 0.2, 0.2); // Strongly red
        let normal_color = Vector3::new(0.5, 0.5, 0.5); // Neutral

        // First red flash should be allowed (use higher intensity to trigger rate limiting)
        // SafetyLevel::Safe multiplies by 0.5, so 0.8 * 0.5 = 0.4 > 0.3 threshold
        assert!(engine.can_allow_effect(0.8, red_color));
        engine.record_effect(0.8, red_color);

        // Immediate second red flash should be blocked (stricter red flash limits)
        assert!(!engine.can_allow_effect(0.8, red_color));

        // But normal color might still be allowed with lower intensity
        assert!(engine.can_allow_effect(0.2, normal_color));
    }

    #[test]
    fn test_safety_multiplier_ranges_are_valid() {
        let multipliers = [
            SafetyMultipliers::ultra_safe(),
            SafetyMultipliers::safe(),
            SafetyMultipliers::moderate(),
            SafetyMultipliers::standard(),
            SafetyMultipliers::disabled(),
            SafetyMultipliers::emergency_stop(),
        ];

        for multiplier in multipliers {
            // All multipliers should be between 0.0 and 1.0
            assert!(multiplier.beat_intensity >= 0.0 && multiplier.beat_intensity <= 1.0);
            assert!(multiplier.onset_intensity >= 0.0 && multiplier.onset_intensity <= 1.0);
            assert!(multiplier.color_change_rate >= 0.0 && multiplier.color_change_rate <= 1.0);
            assert!(multiplier.brightness_range >= 0.0 && multiplier.brightness_range <= 1.0);
            assert!(multiplier.pattern_complexity >= 0.0 && multiplier.pattern_complexity <= 1.0);
        }
    }

    #[test]
    fn test_luminance_calculation_accuracy() {
        // Test ITU-R BT.709 luminance calculation
        let white = Vector3::new(1.0, 1.0, 1.0);
        let black = Vector3::new(0.0, 0.0, 0.0);
        let pure_red = Vector3::new(1.0, 0.0, 0.0);
        let pure_green = Vector3::new(0.0, 1.0, 0.0);
        let pure_blue = Vector3::new(0.0, 0.0, 1.0);

        // White should be 1.0, black should be 0.0
        assert!((LuminanceLimiter::calculate_luminance(white) - 1.0).abs() < 0.001);
        assert!((LuminanceLimiter::calculate_luminance(black) - 0.0).abs() < 0.001);

        // Green should contribute most to luminance (0.7152)
        let green_luminance = LuminanceLimiter::calculate_luminance(pure_green);
        let red_luminance = LuminanceLimiter::calculate_luminance(pure_red);
        let blue_luminance = LuminanceLimiter::calculate_luminance(pure_blue);

        assert!(green_luminance > red_luminance);
        assert!(green_luminance > blue_luminance);
        assert!(red_luminance > blue_luminance);

        // Verify actual values match ITU-R BT.709
        assert!((red_luminance - 0.2126).abs() < 0.001);
        assert!((green_luminance - 0.7152).abs() < 0.001);
        assert!((blue_luminance - 0.0722).abs() < 0.001);
    }

    #[test]
    fn test_safety_integration_with_different_audio_intensities() {
        let mut engine = SafetyEngine::new();
        engine.set_safety_level(SafetyLevel::Safe);

        // Test various audio intensity scenarios
        let low_intensity = 0.1;
        let _medium_intensity = 0.5;
        let high_intensity = 0.9;

        // Low intensity should always be allowed
        assert!(engine.can_allow_effect(low_intensity, Vector3::new(0.5, 0.5, 0.5)));

        // High intensity should be more restricted
        let can_allow_high = engine.can_allow_effect(high_intensity, Vector3::new(0.5, 0.5, 0.5));
        engine.record_effect(high_intensity, Vector3::new(0.5, 0.5, 0.5));

        // If first high intensity was allowed, second should be blocked
        if can_allow_high {
            assert!(!engine.can_allow_effect(high_intensity, Vector3::new(0.5, 0.5, 0.5)));
        }
    }

    #[test]
    fn test_vector3_operations() {
        // Test Vector3 math operations used in safety calculations
        let vec = Vector3::new(0.5, 0.3, 0.2);
        let scalar = 0.8;

        let result = vec * scalar;
        assert!((result.x - 0.4).abs() < 0.001);
        assert!((result.y - 0.24).abs() < 0.001);
        assert!((result.z - 0.16).abs() < 0.001);

        let result2 = vec.mul_scalar(scalar);
        assert_eq!(result.x, result2.x);
        assert_eq!(result.y, result2.y);
        assert_eq!(result.z, result2.z);
    }
}