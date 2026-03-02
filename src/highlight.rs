use syntect::highlighting::{Theme as SyntectTheme, ThemeSet};
use syntect::parsing::SyntaxSet;

use crate::style::{Color, Style};

/// Built-in syntax highlighter powered by syntect.
///
/// Uses the same highlighting engine as Sublime Text and VS Code,
/// supporting 200+ languages out of the box with zero configuration.
pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme: SyntectTheme,
}

impl Highlighter {
    /// Create a highlighter with the default theme (base16-ocean.dark).
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();
        Self { syntax_set, theme }
    }

    /// Create a highlighter with a named syntect theme.
    pub fn with_theme(theme_name: &str) -> Option<Self> {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes.get(theme_name)?.clone();
        Some(Self { syntax_set, theme })
    }

    /// Highlight a block of code, returning lines of styled spans.
    ///
    /// Each line is a `Vec<(Style, String)>` where Style is our ANSI style.
    /// If the language is unknown, returns `None` (caller can fall back to plain).
    pub fn highlight(&self, code: &str, language: &str) -> Option<Vec<Vec<(Style, String)>>> {
        use syntect::easy::HighlightLines;

        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .or_else(|| self.syntax_set.find_syntax_by_extension(language))?;

        let mut h = HighlightLines::new(syntax, &self.theme);
        let mut result = Vec::new();

        for line in syntect::util::LinesWithEndings::from(code) {
            let ranges = h.highlight_line(line, &self.syntax_set).ok()?;
            let styled_line: Vec<(Style, String)> = ranges
                .into_iter()
                .map(|(syntect_style, text)| {
                    let style = syntect_to_mdansi_style(syntect_style);
                    // Strip trailing newline from text (we handle line breaks ourselves)
                    let text = text.trim_end_matches('\n').to_string();
                    (style, text)
                })
                .collect();
            result.push(styled_line);
        }

        Some(result)
    }

    /// Highlight code and render directly to ANSI-escaped string lines.
    pub fn highlight_to_ansi(&self, code: &str, language: &str) -> Option<Vec<String>> {
        let highlighted = self.highlight(code, language)?;
        let lines: Vec<String> = highlighted
            .into_iter()
            .map(|spans| {
                spans
                    .into_iter()
                    .map(|(style, text)| style.paint(&text))
                    .collect::<String>()
            })
            .collect();
        Some(lines)
    }

    /// Check if a language identifier is recognized.
    pub fn supports_language(&self, language: &str) -> bool {
        self.syntax_set.find_syntax_by_token(language).is_some()
            || self.syntax_set.find_syntax_by_extension(language).is_some()
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a syntect style to our ANSI Style.
fn syntect_to_mdansi_style(s: syntect::highlighting::Style) -> Style {
    let fg = Color::Rgb(s.foreground.r, s.foreground.g, s.foreground.b);
    let mut style = Style::new().fg(fg);

    if s.font_style
        .contains(syntect::highlighting::FontStyle::BOLD)
    {
        style.bold = true;
    }
    if s.font_style
        .contains(syntect::highlighting::FontStyle::ITALIC)
    {
        style.italic = true;
    }
    if s.font_style
        .contains(syntect::highlighting::FontStyle::UNDERLINE)
    {
        style.underline = true;
    }

    style
}
