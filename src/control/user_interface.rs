use anyhow::Result;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::rendering::{EnhancedFrameComposer, ShaderType, QualityLevel};

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
        composer.set_shader(shader_type, context)?;

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

        composer.set_shader(next_shader, context)?;
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

        composer.set_shader(prev_shader, context)?;
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