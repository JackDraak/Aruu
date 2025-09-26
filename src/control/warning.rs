/// Photosensitive Epilepsy Warning Screen Implementation
///
/// Provides mandatory user consent and safety information before starting visualization.
/// Complies with international epilepsy prevention standards and gaming industry requirements.

use winit::event::{ElementState, KeyEvent};
use std::time::Instant;

/// Warning screen state and user interaction
#[derive(Debug, Clone, PartialEq)]
pub enum WarningState {
    /// Show initial warning with options
    ShowingWarning,
    /// User selected Safety Mode
    SafetyModeSelected,
    /// User selected Continue (acknowledged risks)
    ContinueSelected,
    /// User selected Exit
    ExitSelected,
    /// Warning dismissed, continue to main app
    Dismissed,
}

/// Epilepsy warning screen manager
pub struct EpilepsyWarning {
    state: WarningState,
    start_time: Instant,
    selected_option: usize, // 0=Continue, 1=Safety Mode, 2=Exit
}

impl EpilepsyWarning {
    pub fn new() -> Self {
        Self {
            state: WarningState::ShowingWarning,
            start_time: Instant::now(),
            selected_option: 1, // Default to Safety Mode
        }
    }

    /// Handle keyboard input for warning screen
    pub fn handle_input(&mut self, key_event: &KeyEvent) -> bool {
        if self.state != WarningState::ShowingWarning {
            return false; // Already dismissed
        }

        if key_event.state != ElementState::Pressed {
            return true; // Consume but don't act on key releases
        }

        match key_event.logical_key.as_ref() {
            // Arrow keys for navigation
            winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft) => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                }
                true
            }
            winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight) => {
                if self.selected_option < 2 {
                    self.selected_option += 1;
                }
                true
            }

            // Number keys for direct selection
            winit::keyboard::Key::Character(c) => {
                let c_str = c.to_string();
                match c_str.as_str() {
                    "1" => {
                        self.selected_option = 0;
                        self.confirm_selection();
                    }
                    "2" => {
                        self.selected_option = 1;
                        self.confirm_selection();
                    }
                    "3" => {
                        self.selected_option = 2;
                        self.confirm_selection();
                    }
                    _ => return true,
                }
                true
            }

            // Enter to confirm selection
            winit::keyboard::Key::Named(winit::keyboard::NamedKey::Enter) => {
                self.confirm_selection();
                true
            }

            // Escape defaults to exit
            winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) => {
                self.selected_option = 2;
                self.confirm_selection();
                true
            }

            _ => true, // Consume all other keys while warning is showing
        }
    }

    /// Confirm the currently selected option
    fn confirm_selection(&mut self) {
        match self.selected_option {
            0 => self.state = WarningState::ContinueSelected,
            1 => self.state = WarningState::SafetyModeSelected,
            2 => self.state = WarningState::ExitSelected,
            _ => self.state = WarningState::ExitSelected, // Default to safe exit
        }
    }

    /// Get current warning state
    pub fn get_state(&self) -> WarningState {
        self.state.clone()
    }

    /// Check if warning should still be displayed
    pub fn should_display(&self) -> bool {
        matches!(self.state, WarningState::ShowingWarning)
    }

    /// Check if user wants to exit
    pub fn should_exit(&self) -> bool {
        matches!(self.state, WarningState::ExitSelected)
    }

    /// Check if user selected safety mode
    pub fn wants_safety_mode(&self) -> bool {
        matches!(self.state, WarningState::SafetyModeSelected)
    }

    /// Dismiss warning and continue
    pub fn dismiss(&mut self) {
        self.state = WarningState::Dismissed;
    }

    /// Get warning text to display
    pub fn get_warning_text(&self) -> String {
        let elapsed = self.start_time.elapsed().as_secs();
        let selection_arrows = match self.selected_option {
            0 => "→ [1] Continue  [ ] Safety Mode  [ ] Exit",
            1 => "[ ] Continue  → [2] Safety Mode  [ ] Exit",
            2 => "[ ] Continue  [ ] Safety Mode  → [3] Exit",
            _ => "[ ] Continue  [ ] Safety Mode  → [3] Exit",
        };

        format!(
r#"
⚠️  PHOTOSENSITIVE EPILEPSY WARNING ⚠️

Aruu Audio Visualizer contains flashing lights and visual effects that may
trigger seizures in individuals with photosensitive epilepsy.

🚨 DO NOT USE if you or anyone in your family has a history of seizures or epilepsy.

Stop using immediately if you experience:
• Dizziness, nausea, or disorientation
• Altered vision or muscle twitching
• Loss of awareness or convulsions

Safety recommendations:
• Use in a well-lit room at least 2 feet from screen
• Take breaks every 30 minutes
• Enable Safety Mode for reduced visual intensity

{}

Controls: Arrow Keys / 1-2-3 / Enter to select / ESC to exit
Time displayed: {}s (minimum 5s required)
"#,
            selection_arrows,
            elapsed
        )
    }

    /// Check if minimum display time has elapsed
    pub fn minimum_time_elapsed(&self) -> bool {
        self.start_time.elapsed().as_secs() >= 5 // Require 5 seconds minimum
    }
}

impl Default for EpilepsyWarning {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_state_transitions() {
        let mut warning = EpilepsyWarning::new();

        assert_eq!(warning.get_state(), WarningState::ShowingWarning);
        assert!(warning.should_display());
        assert!(!warning.should_exit());

        warning.confirm_selection(); // Default is Safety Mode (index 1)
        assert_eq!(warning.get_state(), WarningState::SafetyModeSelected);
        assert!(warning.wants_safety_mode());
    }

    #[test]
    fn test_minimum_display_time() {
        let warning = EpilepsyWarning::new();
        assert!(!warning.minimum_time_elapsed()); // Should be false immediately
    }

    #[test]
    fn test_option_selection() {
        let mut warning = EpilepsyWarning::new();

        // Test exit selection
        warning.selected_option = 2;
        warning.confirm_selection();
        assert!(warning.should_exit());
    }
}