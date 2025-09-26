use anyhow::Result;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::rendering::{EnhancedFrameComposer, ShaderType, QualityLevel};
use crate::control::{SafetyEngine, SafetyLevel, EpilepsyWarning};

/// User interface controls for real-time interaction
pub struct UserInterface {
    /// Enable/disable auto shader selection
    pub auto_shader_enabled: bool,
    /// Manual quality override (None = automatic)
    pub quality_override: Option<QualityLevel>,
    /// Display performance overlay
    pub show_performance_overlay: bool,
    /// Current shader cycling position
    shader_cycle_index: usize,
    /// Available shaders for cycling
    available_shaders: Vec<ShaderType>,
    /// Help display state
    show_help: bool,
    /// Safety system for epilepsy prevention
    pub safety_engine: SafetyEngine,
    /// Epilepsy warning screen handler
    pub epilepsy_warning: EpilepsyWarning,
    /// Current safety level
    current_safety_level: SafetyLevel,
    /// Show safety status in overlay
    pub show_safety_status: bool,
    /// ESC key press tracking for double-press exit
    esc_press_count: u32,
    /// Time of last ESC press for double-press detection
    last_esc_time: std::time::Instant,
    /// Flag to signal application should exit
    should_exit: bool,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            auto_shader_enabled: true,
            quality_override: None,
            show_performance_overlay: false,
            shader_cycle_index: 0,
            available_shaders: vec![
                ShaderType::Classic,
                ShaderType::ParametricWave,
                ShaderType::Plasma,
                ShaderType::Kaleidoscope,
                ShaderType::Tunnel,
                ShaderType::Particle,
                ShaderType::Fractal,
                ShaderType::Spectralizer,
            ],
            show_help: false,
            safety_engine: SafetyEngine::new(),
            epilepsy_warning: EpilepsyWarning::new(),
            current_safety_level: SafetyLevel::Safe, // Default to safe
            show_safety_status: true, // Show safety status by default
            esc_press_count: 0,
            last_esc_time: std::time::Instant::now(),
            should_exit: false,
        }
    }

    /// Handle keyboard input events
    pub fn handle_keyboard_input(
        &mut self,
        event: &KeyEvent,
        composer: &mut EnhancedFrameComposer,
        context: &crate::rendering::WgpuContext,
    ) -> Result<bool> {
        if event.state != ElementState::Pressed {
            return Ok(false);
        }

        let mut handled = false;

        if let PhysicalKey::Code(keycode) = &event.physical_key {
            match keycode {
                // Shader selection (1-8 keys)
                KeyCode::Digit1 => {
                    self.set_shader(ShaderType::Classic, composer, context)?;
                    handled = true;
                }
                KeyCode::Digit2 => {
                    self.set_shader(ShaderType::ParametricWave, composer, context)?;
                    handled = true;
                }
                KeyCode::Digit3 => {
                    self.set_shader(ShaderType::Plasma, composer, context)?;
                    handled = true;
                }
                KeyCode::Digit4 => {
                    self.set_shader(ShaderType::Kaleidoscope, composer, context)?;
                    handled = true;
                }
                KeyCode::Digit5 => {
                    self.set_shader(ShaderType::Tunnel, composer, context)?;
                    handled = true;
                }
                KeyCode::Digit6 => {
                    self.set_shader(ShaderType::Particle, composer, context)?;
                    handled = true;
                }
                KeyCode::Digit7 => {
                    self.set_shader(ShaderType::Fractal, composer, context)?;
                    handled = true;
                }
                KeyCode::Digit8 => {
                    self.set_shader(ShaderType::Spectralizer, composer, context)?;
                    handled = true;
                }

                // Shader cycling
                KeyCode::Space => {
                    self.cycle_next_shader(composer, context)?;
                    handled = true;
                }
                KeyCode::Tab => {
                    self.cycle_previous_shader(composer, context)?;
                    handled = true;
                }

                // Auto shader mode toggle
                KeyCode::KeyA => {
                    self.toggle_auto_shader();
                    handled = true;
                }

                // Quality level controls
                KeyCode::KeyQ => {
                    self.set_quality_override(Some(QualityLevel::Potato), composer);
                    handled = true;
                }
                KeyCode::KeyW => {
                    self.set_quality_override(Some(QualityLevel::Low), composer);
                    handled = true;
                }
                KeyCode::KeyE => {
                    self.set_quality_override(Some(QualityLevel::Medium), composer);
                    handled = true;
                }
                KeyCode::KeyR => {
                    self.set_quality_override(Some(QualityLevel::High), composer);
                    handled = true;
                }
                KeyCode::KeyT => {
                    self.set_quality_override(Some(QualityLevel::Ultra), composer);
                    handled = true;
                }
                KeyCode::KeyY => {
                    self.set_quality_override(None, composer); // Auto quality
                    handled = true;
                }

                // Performance overlay toggle
                KeyCode::KeyP => {
                    self.toggle_performance_overlay();
                    handled = true;
                }

                // Help display toggle
                KeyCode::KeyH | KeyCode::F1 => {
                    self.toggle_help();
                    handled = true;
                }

                // Emergency stop (ESC key) - Critical safety feature with double-press exit
                KeyCode::Escape => {
                    let now = std::time::Instant::now();
                    let time_since_last_esc = now.duration_since(self.last_esc_time).as_secs_f32();

                    if time_since_last_esc <= 2.0 {
                        // Second ESC within 2 seconds - signal exit
                        self.esc_press_count += 1;
                        if self.esc_press_count >= 2 {
                            self.should_exit = true;
                            println!("🚪 Exiting Aruu Audio Visualizer...");
                        }
                    } else {
                        // First ESC or too much time passed - reset and do emergency stop
                        self.esc_press_count = 1;
                        self.emergency_stop();
                    }

                    self.last_esc_time = now;
                    handled = true;
                }

                // Safety level controls
                KeyCode::KeyS => {
                    self.cycle_safety_level();
                    handled = true;
                }

                // Safety status toggle
                KeyCode::KeyZ => {
                    self.toggle_safety_status();
                    handled = true;
                }

                // Resume from emergency stop
                KeyCode::KeyX => {
                    self.resume_from_emergency();
                    handled = true;
                }

                _ => {}
            }
        }

        Ok(handled)
    }

    /// Set specific shader and disable auto mode
    fn set_shader(
        &mut self,
        shader_type: ShaderType,
        composer: &mut EnhancedFrameComposer,
        context: &crate::rendering::WgpuContext,
    ) -> Result<()> {
        self.auto_shader_enabled = false;
        composer.set_shader_immediately(shader_type, context)?;

        // Update cycle index to match current shader
        if let Some(index) = self.available_shaders.iter().position(|&s| s == shader_type) {
            self.shader_cycle_index = index;
        }

        println!("🎨 Manual shader: {} (auto mode disabled)", shader_type.name());
        Ok(())
    }

    /// Cycle to next shader
    fn cycle_next_shader(
        &mut self,
        composer: &mut EnhancedFrameComposer,
        context: &crate::rendering::WgpuContext,
    ) -> Result<()> {
        self.auto_shader_enabled = false;
        self.shader_cycle_index = (self.shader_cycle_index + 1) % self.available_shaders.len();
        let next_shader = self.available_shaders[self.shader_cycle_index];

        composer.set_shader_immediately(next_shader, context)?;
        println!("🔄 Next shader: {} (auto mode disabled)", next_shader.name());
        Ok(())
    }

    /// Cycle to previous shader
    fn cycle_previous_shader(
        &mut self,
        composer: &mut EnhancedFrameComposer,
        context: &crate::rendering::WgpuContext,
    ) -> Result<()> {
        self.auto_shader_enabled = false;
        self.shader_cycle_index = if self.shader_cycle_index == 0 {
            self.available_shaders.len() - 1
        } else {
            self.shader_cycle_index - 1
        };
        let prev_shader = self.available_shaders[self.shader_cycle_index];

        composer.set_shader_immediately(prev_shader, context)?;
        println!("🔄 Previous shader: {} (auto mode disabled)", prev_shader.name());
        Ok(())
    }

    /// Toggle auto shader selection
    fn toggle_auto_shader(&mut self) {
        self.auto_shader_enabled = !self.auto_shader_enabled;
        let status = if self.auto_shader_enabled { "enabled" } else { "disabled" };
        println!("🤖 Auto shader mode: {}", status);
    }

    /// Set quality level override
    fn set_quality_override(&mut self, quality: Option<QualityLevel>, composer: &mut EnhancedFrameComposer) {
        self.quality_override = quality;

        if let Some(q) = quality {
            composer.set_quality(q);
            println!("🔧 Quality override: {:?}", q);
        } else {
            println!("🔧 Quality override: Auto");
        }
    }

    /// Toggle performance overlay
    fn toggle_performance_overlay(&mut self) {
        self.show_performance_overlay = !self.show_performance_overlay;
        let status = if self.show_performance_overlay { "enabled" } else { "disabled" };
        println!("📊 Performance overlay: {}", status);
    }

    /// Toggle help display
    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        if self.show_help {
            self.print_help();
        } else {
            println!("❓ Help hidden");
        }
    }

    /// Print help information
    fn print_help(&self) {
        println!("\n🎵 ARUU - Audio Visualizer Controls 🎵");
        println!("========================================");
        println!("SHADER SELECTION:");
        println!("  1-8     Direct shader selection");
        println!("  Space   Next shader");
        println!("  Tab     Previous shader");
        println!("  A       Toggle auto shader mode");
        println!();
        println!("QUALITY CONTROL:");
        println!("  Q       Potato quality");
        println!("  W       Low quality");
        println!("  E       Medium quality");
        println!("  R       High quality");
        println!("  T       Ultra quality");
        println!("  Y       Auto quality");
        println!();
        println!("🛡️  SAFETY CONTROLS:");
        println!("  ESC     Emergency stop (critical safety)");
        println!("  S       Cycle safety level");
        println!("  X       Resume from emergency stop");
        println!("  Z       Toggle safety status display");
        println!();
        println!("DISPLAY:");
        println!("  P       Toggle performance overlay");
        println!("  H/F1    Toggle this help");
        println!();
        println!("SHADERS:");
        println!("  1. Classic      - Original wave patterns");
        println!("  2. Parametric   - Mathematical audio-reactive patterns");
        println!("  3. Plasma       - Fluid organic patterns");
        println!("  4. Kaleidoscope - Symmetric patterns");
        println!("  5. Tunnel       - 3D perspective effects");
        println!("  6. Particle     - Dynamic particle systems");
        println!("  7. Fractal      - Mandelbrot/Julia sets");
        println!("  8. Spectralizer - Direct frequency visualization");
        println!();
        println!("🛡️  SAFETY LEVELS:");
        println!("  🛡️ Ultra Safe   - Maximum epilepsy protection");
        println!("  🔒 Safe         - Conservative (default)");
        println!("  ⚠️ Moderate     - Balanced experience");
        println!("  🎨 Standard     - Near-full features");
        println!("========================================\n");
    }

    /// Get current control status for display
    pub fn get_status_text(&self, composer: &EnhancedFrameComposer) -> String {
        let shader_status = if self.auto_shader_enabled {
            format!("AUTO ({})", composer.current_shader().name())
        } else {
            format!("MANUAL ({})", composer.current_shader().name())
        };

        let quality_status = match self.quality_override {
            Some(q) => format!("{:?}", q),
            None => format!("AUTO ({:?})", composer.current_quality()),
        };

        format!(
            "Shader: {} | Quality: {} | FPS: {:.1}",
            shader_status,
            quality_status,
            composer.average_fps()
        )
    }

    /// Get performance overlay text
    pub fn get_performance_overlay(&self, composer: &EnhancedFrameComposer) -> Option<String> {
        if self.show_performance_overlay {
            Some(format!(
                "🔍 PERFORMANCE METRICS\n{}\n📊 {}",
                "=" .repeat(40),
                composer.performance_report()
            ))
        } else {
            None
        }
    }

    /// Check if auto shader mode is enabled
    pub fn is_auto_shader_enabled(&self) -> bool {
        self.auto_shader_enabled
    }

    /// Get current shader cycle index
    pub fn current_shader_index(&self) -> usize {
        self.shader_cycle_index
    }

    // ====== SAFETY CONTROL METHODS ======

    /// Emergency stop - immediately halt all visual effects
    pub fn emergency_stop(&mut self) {
        self.safety_engine.emergency_stop();
        println!("⛔ EMERGENCY STOP ACTIVATED - All visual effects halted");
        println!("   Press X to resume or adjust safety levels");
        println!("   ESC again to exit application");
    }

    /// Resume from emergency stop
    pub fn resume_from_emergency(&mut self) {
        if self.safety_engine.is_emergency_stopped() {
            self.safety_engine.resume();
            println!("✅ Emergency stop released - Visual effects resumed");
            println!("   Current safety level: {:?}", self.current_safety_level);
        }
    }

    /// Cycle through safety levels
    pub fn cycle_safety_level(&mut self) {
        self.current_safety_level = match self.current_safety_level {
            SafetyLevel::UltraSafe => SafetyLevel::Safe,
            SafetyLevel::Safe => SafetyLevel::Moderate,
            SafetyLevel::Moderate => SafetyLevel::Standard,
            SafetyLevel::Standard => SafetyLevel::UltraSafe, // Loop back to most safe
            SafetyLevel::Disabled => SafetyLevel::UltraSafe, // Never stay disabled from user input
        };

        self.safety_engine.set_safety_level(self.current_safety_level);

        let level_description = match self.current_safety_level {
            SafetyLevel::UltraSafe => "🛡️ Ultra Safe (Maximum protection)",
            SafetyLevel::Safe => "🔒 Safe (Conservative default)",
            SafetyLevel::Moderate => "⚠️ Moderate (Balanced experience)",
            SafetyLevel::Standard => "🎨 Standard (Near-full features)",
            SafetyLevel::Disabled => "⚠️ DISABLED (Medical use only)",
        };

        println!("🛡️  Safety Level: {}", level_description);
    }

    /// Toggle safety status display
    pub fn toggle_safety_status(&mut self) {
        self.show_safety_status = !self.show_safety_status;
        let status = if self.show_safety_status { "ON" } else { "OFF" };
        println!("🔍 Safety status display: {}", status);
    }

    /// Get current safety level
    pub fn get_safety_level(&self) -> SafetyLevel {
        self.current_safety_level
    }

    /// Get safety engine for external access
    pub fn get_safety_engine(&self) -> &SafetyEngine {
        &self.safety_engine
    }

    /// Get mutable safety engine for external access
    pub fn get_safety_engine_mut(&mut self) -> &mut SafetyEngine {
        &mut self.safety_engine
    }

    /// Check if emergency stop is active
    pub fn is_emergency_stopped(&self) -> bool {
        self.safety_engine.is_emergency_stopped()
    }

    /// Check if application should exit (double ESC press detected)
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    /// Get current safety multipliers for shaders
    pub fn get_safety_multipliers(&self) -> crate::control::safety::SafetyMultipliers {
        self.safety_engine.get_safety_multipliers()
    }

    /// Handle epilepsy warning screen input
    pub fn handle_warning_input(&mut self, event: &KeyEvent) -> bool {
        self.epilepsy_warning.handle_input(event)
    }

    /// Check if warning should be displayed
    pub fn should_display_warning(&self) -> bool {
        self.epilepsy_warning.should_display()
    }

    /// Check if user wants to exit from warning
    pub fn should_exit_from_warning(&self) -> bool {
        self.epilepsy_warning.should_exit()
    }

    /// Apply safety mode from warning selection
    pub fn apply_warning_selection(&mut self) {
        if self.epilepsy_warning.wants_safety_mode() {
            self.current_safety_level = SafetyLevel::UltraSafe;
            self.safety_engine.set_safety_level(SafetyLevel::UltraSafe);
            println!("🛡️  Ultra Safe mode activated from warning screen");
        }
        self.epilepsy_warning.dismiss();
    }

    /// Get safety status for display in performance overlay
    pub fn get_safety_status_display(&self) -> Option<String> {
        if !self.show_safety_status {
            return None;
        }

        let status = self.safety_engine.get_safety_status();
        let mut safety_text = vec![
            "🛡️  SAFETY STATUS".to_string(),
            "=".repeat(20),
            status.get_status_message(),
        ];

        if status.emergency_stopped {
            safety_text.push("⛔ EMERGENCY STOP ACTIVE".to_string());
            safety_text.push("Press X to resume".to_string());
        }

        if status.should_warn_user() {
            safety_text.push("⚠️  High visual activity detected".to_string());
        }

        if !status.warnings.is_empty() {
            for warning in &status.warnings {
                safety_text.push(format!("⚠️  {}", warning));
            }
        }

        Some(safety_text.join("\n"))
    }
}

impl Default for UserInterface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_interface_creation() {
        let ui = UserInterface::new();
        assert!(ui.auto_shader_enabled);
        assert!(ui.quality_override.is_none());
        assert!(!ui.show_performance_overlay);
        assert_eq!(ui.available_shaders.len(), 8);
    }

    #[test]
    fn test_shader_cycling() {
        let mut ui = UserInterface::new();

        // Test cycling forward
        let initial_index = ui.shader_cycle_index;
        let expected_next = (initial_index + 1) % ui.available_shaders.len();

        // Note: We can't test the actual cycling without a composer and context,
        // but we can test the index logic
        assert_eq!(ui.current_shader_index(), initial_index);
    }

    #[test]
    fn test_auto_shader_toggle() {
        let mut ui = UserInterface::new();
        assert!(ui.is_auto_shader_enabled());

        ui.toggle_auto_shader();
        assert!(!ui.is_auto_shader_enabled());

        ui.toggle_auto_shader();
        assert!(ui.is_auto_shader_enabled());
    }

    #[test]
    fn test_quality_override() {
        let mut ui = UserInterface::new();
        assert!(ui.quality_override.is_none());

        ui.quality_override = Some(QualityLevel::High);
        assert_eq!(ui.quality_override, Some(QualityLevel::High));
    }

    #[test]
    fn test_performance_overlay_toggle() {
        let mut ui = UserInterface::new();
        assert!(!ui.show_performance_overlay);

        ui.toggle_performance_overlay();
        assert!(ui.show_performance_overlay);

        ui.toggle_performance_overlay();
        assert!(!ui.show_performance_overlay);
    }
}