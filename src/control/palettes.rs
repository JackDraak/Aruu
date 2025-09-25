#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorPalette {
    Rainbow = 0,
    Red = 1,
    Orange = 2,
    Yellow = 3,
    Green = 4,
    Blue = 5,
    Indigo = 6,
    Violet = 7,
}

impl ColorPalette {
    pub const COUNT: usize = 8;

    pub fn all_palettes() -> [ColorPalette; Self::COUNT] {
        [
            ColorPalette::Rainbow,
            ColorPalette::Red,
            ColorPalette::Orange,
            ColorPalette::Yellow,
            ColorPalette::Green,
            ColorPalette::Blue,
            ColorPalette::Indigo,
            ColorPalette::Violet,
        ]
    }

    pub fn next(&self) -> ColorPalette {
        let palettes = Self::all_palettes();
        let current_index = *self as usize;
        palettes[(current_index + 1) % Self::COUNT]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ColorPalette::Rainbow => "Rainbow",
            ColorPalette::Red => "Red",
            ColorPalette::Orange => "Orange",
            ColorPalette::Yellow => "Yellow",
            ColorPalette::Green => "Green",
            ColorPalette::Blue => "Blue",
            ColorPalette::Indigo => "Indigo",
            ColorPalette::Violet => "Violet",
        }
    }

    pub fn base_hue(&self) -> f32 {
        match self {
            ColorPalette::Rainbow => 0.0,    // Will be overridden in shader
            ColorPalette::Red => 0.0,        // 0°
            ColorPalette::Orange => 0.083,   // 30°
            ColorPalette::Yellow => 0.167,   // 60°
            ColorPalette::Green => 0.333,    // 120°
            ColorPalette::Blue => 0.667,     // 240°
            ColorPalette::Indigo => 0.75,    // 270°
            ColorPalette::Violet => 0.833,   // 300°
        }
    }

    pub fn hue_range(&self) -> f32 {
        match self {
            ColorPalette::Rainbow => 1.0,    // Full spectrum
            ColorPalette::Red => 0.083,      // ±30° around red
            ColorPalette::Orange => 0.083,   // ±30° around orange
            ColorPalette::Yellow => 0.083,   // ±30° around yellow
            ColorPalette::Green => 0.167,    // ±60° around green (more variation)
            ColorPalette::Blue => 0.167,     // ±60° around blue (more variation)
            ColorPalette::Indigo => 0.083,   // ±30° around indigo
            ColorPalette::Violet => 0.083,   // ±30° around violet
        }
    }

    pub fn as_index(&self) -> f32 {
        *self as usize as f32
    }
}

pub struct PaletteManager {
    current_palette: ColorPalette,
    previous_palette: ColorPalette,
    switch_cooldown: f32,
    last_switch_time: f32,
    transition_duration: f32,
    in_transition: bool,
}

impl PaletteManager {
    pub fn new() -> Self {
        Self {
            current_palette: ColorPalette::Rainbow,
            previous_palette: ColorPalette::Rainbow,
            switch_cooldown: 2.0, // Minimum seconds between palette switches (longer for downbeats)
            last_switch_time: 0.0,
            transition_duration: 1.0, // 1 second cross-fade
            in_transition: false,
        }
    }

    pub fn current_palette(&self) -> ColorPalette {
        self.current_palette
    }

    pub fn try_switch_palette(&mut self, current_time: f32, downbeat_detected: bool) -> bool {
        if downbeat_detected && (current_time - self.last_switch_time) >= self.switch_cooldown {
            self.previous_palette = self.current_palette;
            self.current_palette = self.current_palette.next();
            self.last_switch_time = current_time;
            self.in_transition = true;
            println!("🎵 Palette cross-fading on downbeat to: {}", self.current_palette.name());
            true
        } else {
            false
        }
    }

    pub fn get_transition_blend(&self, current_time: f32) -> f32 {
        if !self.in_transition {
            return 1.0; // No transition, fully showing current palette
        }

        let elapsed = current_time - self.last_switch_time;
        if elapsed >= self.transition_duration {
            return 1.0; // Transition complete
        }

        // Smooth transition curve (ease-in-out)
        let t = elapsed / self.transition_duration;
        let smooth_t = t * t * (3.0 - 2.0 * t); // Smoothstep
        smooth_t
    }

    pub fn update_transition(&mut self, current_time: f32) {
        if self.in_transition && (current_time - self.last_switch_time) >= self.transition_duration {
            self.in_transition = false;
        }
    }

    pub fn previous_palette(&self) -> ColorPalette {
        self.previous_palette
    }

    pub fn set_cooldown(&mut self, seconds: f32) {
        self.switch_cooldown = seconds.max(0.1);
    }

    pub fn force_switch_palette(&mut self, palette: ColorPalette, current_time: f32) {
        self.current_palette = palette;
        self.last_switch_time = current_time;
        println!("🎨 Palette forced to: {}", palette.name());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_cycling() {
        let rainbow = ColorPalette::Rainbow;
        let next = rainbow.next();
        assert_eq!(next, ColorPalette::Red);

        let violet = ColorPalette::Violet;
        let back_to_rainbow = violet.next();
        assert_eq!(back_to_rainbow, ColorPalette::Rainbow);
    }

    #[test]
    fn test_palette_manager() {
        let mut manager = PaletteManager::new();
        assert_eq!(manager.current_palette(), ColorPalette::Rainbow);

        // Should not switch immediately due to cooldown
        let switched = manager.try_switch_palette(0.1, true);
        assert!(!switched);
        assert_eq!(manager.current_palette(), ColorPalette::Rainbow);

        // Should switch after cooldown (increased from 1.0 to 3.0 due to new 2.0s cooldown)
        let switched = manager.try_switch_palette(3.0, true);
        assert!(switched);
        assert_eq!(manager.current_palette(), ColorPalette::Red);
    }

    #[test]
    fn test_palette_properties() {
        assert_eq!(ColorPalette::Rainbow.name(), "Rainbow");
        assert_eq!(ColorPalette::Red.base_hue(), 0.0);
        assert_eq!(ColorPalette::Green.base_hue(), 0.333);
        assert_eq!(ColorPalette::Rainbow.hue_range(), 1.0);
        assert_eq!(ColorPalette::Red.hue_range(), 0.083);
    }
}