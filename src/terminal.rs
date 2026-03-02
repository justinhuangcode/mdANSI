use crate::style::ColorLevel;

/// Detected terminal capabilities.
#[derive(Debug, Clone)]
pub struct TerminalCaps {
    pub width: usize,
    pub color_level: ColorLevel,
    pub hyperlinks: bool,
    pub is_tty: bool,
    pub sixel: bool,
    pub kitty_graphics: bool,
}

impl Default for TerminalCaps {
    fn default() -> Self {
        Self {
            width: 80,
            color_level: ColorLevel::None,
            hyperlinks: false,
            is_tty: false,
            sixel: false,
            kitty_graphics: false,
        }
    }
}

impl TerminalCaps {
    /// Auto-detect terminal capabilities from the environment.
    pub fn detect() -> Self {
        let is_tty = crossterm::tty::IsTty::is_tty(&std::io::stdout());
        let width = crossterm::terminal::size()
            .map(|(w, _)| w as usize)
            .unwrap_or(80);
        let color_level = detect_color_level(is_tty);
        let hyperlinks = detect_hyperlinks();

        Self {
            width,
            color_level,
            hyperlinks,
            is_tty,
            sixel: detect_sixel(),
            kitty_graphics: detect_kitty_graphics(),
        }
    }

    /// Create capabilities for a non-interactive pipe (no colors, no TTY).
    pub fn pipe(width: usize) -> Self {
        Self {
            width,
            ..Self::default()
        }
    }

    /// Override width.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Override color level.
    pub fn with_color_level(mut self, level: ColorLevel) -> Self {
        self.color_level = level;
        self
    }

    /// Check if any color output is supported.
    pub fn has_color(&self) -> bool {
        self.color_level > ColorLevel::None
    }
}

fn detect_color_level(is_tty: bool) -> ColorLevel {
    // Respect NO_COLOR convention (https://no-color.org/)
    if std::env::var_os("NO_COLOR").is_some() {
        return ColorLevel::None;
    }

    // FORCE_COLOR overrides TTY detection
    if let Ok(val) = std::env::var("FORCE_COLOR") {
        return match val.as_str() {
            "0" => ColorLevel::None,
            "1" => ColorLevel::Basic,
            "2" => ColorLevel::Palette,
            "3" => ColorLevel::TrueColor,
            _ => ColorLevel::TrueColor,
        };
    }

    if !is_tty {
        return ColorLevel::None;
    }

    // Check COLORTERM for truecolor support
    if let Ok(ct) = std::env::var("COLORTERM") {
        if ct == "truecolor" || ct == "24bit" {
            return ColorLevel::TrueColor;
        }
    }

    // Check common terminal emulators known to support truecolor
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        match term_program.as_str() {
            "iTerm.app" | "WezTerm" | "Hyper" | "vscode" => return ColorLevel::TrueColor,
            _ => {}
        }
    }

    // Check TERM for 256-color
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("256color") {
            return ColorLevel::Palette;
        }
        if term == "dumb" {
            return ColorLevel::None;
        }
    }

    // Default: basic colors for TTY
    ColorLevel::Basic
}

fn detect_hyperlinks() -> bool {
    // Known terminals that support OSC-8 hyperlinks
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        match term_program.as_str() {
            "iTerm.app" | "WezTerm" | "vscode" => return true,
            _ => {}
        }
    }

    // VTE-based terminals (GNOME Terminal, Tilix, etc.)
    std::env::var_os("VTE_VERSION").is_some()
}

fn detect_sixel() -> bool {
    // Heuristic: check TERM_PROGRAM or known env vars
    if let Ok(tp) = std::env::var("TERM_PROGRAM") {
        if tp == "mlterm" || tp == "XTerm" {
            return true;
        }
    }
    false
}

fn detect_kitty_graphics() -> bool {
    std::env::var("KITTY_WINDOW_ID").is_ok()
}
