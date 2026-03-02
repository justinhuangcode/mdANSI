use std::fmt;

/// ANSI color representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    /// Standard 4-bit colors (0-15).
    Ansi(u8),
    /// 256-color palette index.
    Palette(u8),
    /// 24-bit true color.
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn fg_code(&self) -> String {
        match self {
            Color::Ansi(n) if *n < 8 => format!("\x1b[{}m", 30 + n),
            Color::Ansi(n) => format!("\x1b[{}m", 82 + n), // bright: 90-97
            Color::Palette(n) => format!("\x1b[38;5;{}m", n),
            Color::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }

    pub fn bg_code(&self) -> String {
        match self {
            Color::Ansi(n) if *n < 8 => format!("\x1b[{}m", 40 + n),
            Color::Ansi(n) => format!("\x1b[{}m", 92 + n), // bright bg: 100-107
            Color::Palette(n) => format!("\x1b[48;5;{}m", n),
            Color::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }

    /// Parse a hex color string like "#ff5733" or "ff5733".
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    }
}

/// Text style attributes that can be combined.
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Returns true if this style has no attributes set.
    pub fn is_empty(&self) -> bool {
        self.fg.is_none()
            && self.bg.is_none()
            && !self.bold
            && !self.italic
            && !self.underline
            && !self.dim
            && !self.strikethrough
    }

    /// Generate the ANSI escape sequence to activate this style.
    pub fn open(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut codes = String::new();
        if let Some(ref fg) = self.fg {
            codes.push_str(&fg.fg_code());
        }
        if let Some(ref bg) = self.bg {
            codes.push_str(&bg.bg_code());
        }
        if self.bold {
            codes.push_str("\x1b[1m");
        }
        if self.dim {
            codes.push_str("\x1b[2m");
        }
        if self.italic {
            codes.push_str("\x1b[3m");
        }
        if self.underline {
            codes.push_str("\x1b[4m");
        }
        if self.strikethrough {
            codes.push_str("\x1b[9m");
        }
        codes
    }

    /// Generate the ANSI escape sequence to reset this style.
    pub fn close(&self) -> &'static str {
        if self.is_empty() {
            ""
        } else {
            "\x1b[0m"
        }
    }

    /// Wrap text with this style's ANSI codes.
    pub fn paint(&self, text: &str) -> String {
        if self.is_empty() || text.is_empty() {
            return text.to_string();
        }
        format!("{}{}{}", self.open(), text, self.close())
    }

    /// Merge another style on top of this one (overlay takes precedence).
    pub fn merge(&self, overlay: &Style) -> Style {
        Style {
            fg: overlay.fg.clone().or_else(|| self.fg.clone()),
            bg: overlay.bg.clone().or_else(|| self.bg.clone()),
            bold: overlay.bold || self.bold,
            italic: overlay.italic || self.italic,
            underline: overlay.underline || self.underline,
            dim: overlay.dim || self.dim,
            strikethrough: overlay.strikethrough || self.strikethrough,
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.open())
    }
}

/// A styled span of text. The fundamental unit of rendered output.
#[derive(Debug, Clone)]
pub struct StyledSpan {
    pub text: String,
    pub style: Style,
}

impl StyledSpan {
    pub fn new(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
        }
    }

    pub fn render(&self) -> String {
        self.style.paint(&self.text)
    }
}

/// Color support level detected for the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorLevel {
    /// No color support.
    None,
    /// Basic 16 colors.
    Basic,
    /// 256-color palette.
    Palette,
    /// 24-bit true color.
    TrueColor,
}

pub const RESET: &str = "\x1b[0m";
