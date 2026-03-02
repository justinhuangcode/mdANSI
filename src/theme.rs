use serde::Deserialize;

use crate::style::{Color, Style};

/// Semantic style intents — what we want to style, not how.
/// The theme maps these intents to concrete ANSI styles.
#[derive(Debug, Clone)]
pub struct Theme {
    pub heading1: Style,
    pub heading2: Style,
    pub heading3: Style,
    pub heading4: Style,
    pub heading5: Style,
    pub heading6: Style,
    pub paragraph: Style,
    pub emphasis: Style,
    pub strong: Style,
    pub strikethrough: Style,
    pub inline_code: Style,
    pub code_block: Style,
    pub code_border: Style,
    pub code_lang_label: Style,
    pub code_line_number: Style,
    pub blockquote: Style,
    pub blockquote_bar: Style,
    pub link_text: Style,
    pub link_url: Style,
    pub list_bullet: Style,
    pub list_number: Style,
    pub task_done: Style,
    pub task_pending: Style,
    pub table_border: Style,
    pub table_header: Style,
    pub table_cell: Style,
    pub thematic_break: Style,
    pub image_alt: Style,
    pub footnote_ref: Style,
    pub footnote_def: Style,
}

impl Default for Theme {
    fn default() -> Self {
        default_theme()
    }
}

/// TOML-serializable theme definition.
#[derive(Debug, Deserialize, Default)]
pub struct ThemeFile {
    #[serde(default)]
    pub heading1: ThemeStyleDef,
    #[serde(default)]
    pub heading2: ThemeStyleDef,
    #[serde(default)]
    pub heading3: ThemeStyleDef,
    #[serde(default)]
    pub heading4: ThemeStyleDef,
    #[serde(default)]
    pub heading5: ThemeStyleDef,
    #[serde(default)]
    pub heading6: ThemeStyleDef,
    #[serde(default)]
    pub paragraph: ThemeStyleDef,
    #[serde(default)]
    pub emphasis: ThemeStyleDef,
    #[serde(default)]
    pub strong: ThemeStyleDef,
    #[serde(default)]
    pub strikethrough: ThemeStyleDef,
    #[serde(default)]
    pub inline_code: ThemeStyleDef,
    #[serde(default)]
    pub code_block: ThemeStyleDef,
    #[serde(default)]
    pub code_border: ThemeStyleDef,
    #[serde(default)]
    pub code_lang_label: ThemeStyleDef,
    #[serde(default)]
    pub code_line_number: ThemeStyleDef,
    #[serde(default)]
    pub blockquote: ThemeStyleDef,
    #[serde(default)]
    pub blockquote_bar: ThemeStyleDef,
    #[serde(default)]
    pub link_text: ThemeStyleDef,
    #[serde(default)]
    pub link_url: ThemeStyleDef,
    #[serde(default)]
    pub list_bullet: ThemeStyleDef,
    #[serde(default)]
    pub list_number: ThemeStyleDef,
    #[serde(default)]
    pub task_done: ThemeStyleDef,
    #[serde(default)]
    pub task_pending: ThemeStyleDef,
    #[serde(default)]
    pub table_border: ThemeStyleDef,
    #[serde(default)]
    pub table_header: ThemeStyleDef,
    #[serde(default)]
    pub table_cell: ThemeStyleDef,
    #[serde(default)]
    pub thematic_break: ThemeStyleDef,
    #[serde(default)]
    pub image_alt: ThemeStyleDef,
    #[serde(default)]
    pub footnote_ref: ThemeStyleDef,
    #[serde(default)]
    pub footnote_def: ThemeStyleDef,
}

/// TOML definition for a single style slot.
#[derive(Debug, Deserialize, Default)]
pub struct ThemeStyleDef {
    pub fg: Option<String>,
    pub bg: Option<String>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub dim: bool,
    #[serde(default)]
    pub strikethrough: bool,
}

impl ThemeStyleDef {
    fn to_style(&self) -> Style {
        Style {
            fg: self.fg.as_deref().and_then(parse_color),
            bg: self.bg.as_deref().and_then(parse_color),
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            dim: self.dim,
            strikethrough: self.strikethrough,
        }
    }
}

impl ThemeFile {
    /// Load a theme from a TOML file, overlaid on top of the default theme.
    pub fn load(path: &std::path::Path) -> crate::error::Result<Theme> {
        let content = std::fs::read_to_string(path)?;
        let file: ThemeFile = toml::from_str(&content)?;
        Ok(file.into_theme())
    }

    /// Convert the file definition into a full Theme, using defaults for unspecified fields.
    pub fn into_theme(self) -> Theme {
        let base = default_theme();
        Theme {
            heading1: overlay(&base.heading1, &self.heading1),
            heading2: overlay(&base.heading2, &self.heading2),
            heading3: overlay(&base.heading3, &self.heading3),
            heading4: overlay(&base.heading4, &self.heading4),
            heading5: overlay(&base.heading5, &self.heading5),
            heading6: overlay(&base.heading6, &self.heading6),
            paragraph: overlay(&base.paragraph, &self.paragraph),
            emphasis: overlay(&base.emphasis, &self.emphasis),
            strong: overlay(&base.strong, &self.strong),
            strikethrough: overlay(&base.strikethrough, &self.strikethrough),
            inline_code: overlay(&base.inline_code, &self.inline_code),
            code_block: overlay(&base.code_block, &self.code_block),
            code_border: overlay(&base.code_border, &self.code_border),
            code_lang_label: overlay(&base.code_lang_label, &self.code_lang_label),
            code_line_number: overlay(&base.code_line_number, &self.code_line_number),
            blockquote: overlay(&base.blockquote, &self.blockquote),
            blockquote_bar: overlay(&base.blockquote_bar, &self.blockquote_bar),
            link_text: overlay(&base.link_text, &self.link_text),
            link_url: overlay(&base.link_url, &self.link_url),
            list_bullet: overlay(&base.list_bullet, &self.list_bullet),
            list_number: overlay(&base.list_number, &self.list_number),
            task_done: overlay(&base.task_done, &self.task_done),
            task_pending: overlay(&base.task_pending, &self.task_pending),
            table_border: overlay(&base.table_border, &self.table_border),
            table_header: overlay(&base.table_header, &self.table_header),
            table_cell: overlay(&base.table_cell, &self.table_cell),
            thematic_break: overlay(&base.thematic_break, &self.thematic_break),
            image_alt: overlay(&base.image_alt, &self.image_alt),
            footnote_ref: overlay(&base.footnote_ref, &self.footnote_ref),
            footnote_def: overlay(&base.footnote_def, &self.footnote_def),
        }
    }
}

/// If the TOML def has any field set, use it; otherwise keep the base style.
fn overlay(base: &Style, def: &ThemeStyleDef) -> Style {
    let over = def.to_style();
    if over.is_empty() {
        base.clone()
    } else {
        over
    }
}

fn parse_color(s: &str) -> Option<Color> {
    // Named colors
    match s.to_lowercase().as_str() {
        "black" => return Some(Color::Ansi(0)),
        "red" => return Some(Color::Ansi(1)),
        "green" => return Some(Color::Ansi(2)),
        "yellow" => return Some(Color::Ansi(3)),
        "blue" => return Some(Color::Ansi(4)),
        "magenta" => return Some(Color::Ansi(5)),
        "cyan" => return Some(Color::Ansi(6)),
        "white" => return Some(Color::Ansi(7)),
        "bright_black" | "gray" | "grey" => return Some(Color::Ansi(8)),
        "bright_red" => return Some(Color::Ansi(9)),
        "bright_green" => return Some(Color::Ansi(10)),
        "bright_yellow" => return Some(Color::Ansi(11)),
        "bright_blue" => return Some(Color::Ansi(12)),
        "bright_magenta" => return Some(Color::Ansi(13)),
        "bright_cyan" => return Some(Color::Ansi(14)),
        "bright_white" => return Some(Color::Ansi(15)),
        _ => {}
    }

    // Hex color
    if s.starts_with('#') {
        return Color::from_hex(s);
    }

    // 256-color index
    if let Ok(n) = s.parse::<u8>() {
        return Some(Color::Palette(n));
    }

    None
}

// ──────────────────────────────────────────────────────────────────────────────
// Built-in themes
// ──────────────────────────────────────────────────────────────────────────────

pub fn default_theme() -> Theme {
    Theme {
        heading1: Style::new().fg(Color::Ansi(3)).bold(), // yellow bold
        heading2: Style::new().fg(Color::Ansi(2)).bold(), // green bold
        heading3: Style::new().fg(Color::Ansi(6)).bold(), // cyan bold
        heading4: Style::new().fg(Color::Ansi(4)).bold(), // blue bold
        heading5: Style::new().fg(Color::Ansi(5)).bold(), // magenta bold
        heading6: Style::new().fg(Color::Ansi(5)).bold().dim(), // magenta dim bold
        paragraph: Style::new(),
        emphasis: Style::new().italic(),
        strong: Style::new().bold(),
        strikethrough: Style::new().strikethrough().dim(),
        inline_code: Style::new().fg(Color::Ansi(6)), // cyan
        code_block: Style::new().fg(Color::Ansi(2)),  // green
        code_border: Style::new().dim(),
        code_lang_label: Style::new().dim().italic(),
        code_line_number: Style::new().dim(),
        blockquote: Style::new().italic().dim(),
        blockquote_bar: Style::new().fg(Color::Ansi(8)), // gray
        link_text: Style::new().fg(Color::Ansi(4)).underline(), // blue underline
        link_url: Style::new().fg(Color::Ansi(8)),       // gray
        list_bullet: Style::new().fg(Color::Ansi(6)),    // cyan
        list_number: Style::new().fg(Color::Ansi(6)),    // cyan
        task_done: Style::new().fg(Color::Ansi(2)),      // green
        task_pending: Style::new().fg(Color::Ansi(8)),   // gray
        table_border: Style::new().dim(),
        table_header: Style::new().bold(),
        table_cell: Style::new(),
        thematic_break: Style::new().dim(),
        image_alt: Style::new().italic().fg(Color::Ansi(5)), // magenta italic
        footnote_ref: Style::new().fg(Color::Ansi(6)).dim(),
        footnote_def: Style::new().dim(),
    }
}

pub fn solarized_theme() -> Theme {
    Theme {
        heading1: Style::new().fg(Color::Rgb(181, 137, 0)).bold(), // yellow
        heading2: Style::new().fg(Color::Rgb(133, 153, 0)).bold(), // green
        heading3: Style::new().fg(Color::Rgb(42, 161, 152)).bold(), // cyan
        heading4: Style::new().fg(Color::Rgb(38, 139, 210)).bold(), // blue
        heading5: Style::new().fg(Color::Rgb(108, 113, 196)).bold(), // violet
        heading6: Style::new().fg(Color::Rgb(211, 54, 130)).bold(), // magenta
        paragraph: Style::new().fg(Color::Rgb(131, 148, 150)),     // base0
        emphasis: Style::new().fg(Color::Rgb(131, 148, 150)).italic(),
        strong: Style::new().fg(Color::Rgb(147, 161, 161)).bold(), // base1
        strikethrough: Style::new().fg(Color::Rgb(88, 110, 117)).strikethrough(),
        inline_code: Style::new().fg(Color::Rgb(42, 161, 152)),
        code_block: Style::new().fg(Color::Rgb(131, 148, 150)),
        code_border: Style::new().fg(Color::Rgb(88, 110, 117)),
        code_lang_label: Style::new().fg(Color::Rgb(88, 110, 117)).italic(),
        code_line_number: Style::new().fg(Color::Rgb(88, 110, 117)),
        blockquote: Style::new().fg(Color::Rgb(88, 110, 117)).italic(),
        blockquote_bar: Style::new().fg(Color::Rgb(88, 110, 117)),
        link_text: Style::new().fg(Color::Rgb(38, 139, 210)).underline(),
        link_url: Style::new().fg(Color::Rgb(88, 110, 117)),
        list_bullet: Style::new().fg(Color::Rgb(42, 161, 152)),
        list_number: Style::new().fg(Color::Rgb(42, 161, 152)),
        task_done: Style::new().fg(Color::Rgb(133, 153, 0)),
        task_pending: Style::new().fg(Color::Rgb(88, 110, 117)),
        table_border: Style::new().fg(Color::Rgb(88, 110, 117)),
        table_header: Style::new().fg(Color::Rgb(147, 161, 161)).bold(),
        table_cell: Style::new().fg(Color::Rgb(131, 148, 150)),
        thematic_break: Style::new().fg(Color::Rgb(88, 110, 117)),
        image_alt: Style::new().fg(Color::Rgb(108, 113, 196)).italic(),
        footnote_ref: Style::new().fg(Color::Rgb(42, 161, 152)),
        footnote_def: Style::new().fg(Color::Rgb(88, 110, 117)),
    }
}

pub fn dracula_theme() -> Theme {
    Theme {
        heading1: Style::new().fg(Color::Rgb(189, 147, 249)).bold(), // purple
        heading2: Style::new().fg(Color::Rgb(80, 250, 123)).bold(),  // green
        heading3: Style::new().fg(Color::Rgb(139, 233, 253)).bold(), // cyan
        heading4: Style::new().fg(Color::Rgb(255, 121, 198)).bold(), // pink
        heading5: Style::new().fg(Color::Rgb(255, 184, 108)).bold(), // orange
        heading6: Style::new().fg(Color::Rgb(241, 250, 140)).bold(), // yellow
        paragraph: Style::new().fg(Color::Rgb(248, 248, 242)),       // foreground
        emphasis: Style::new().fg(Color::Rgb(248, 248, 242)).italic(),
        strong: Style::new().fg(Color::Rgb(255, 184, 108)).bold(),
        strikethrough: Style::new().fg(Color::Rgb(98, 114, 164)).strikethrough(),
        inline_code: Style::new().fg(Color::Rgb(139, 233, 253)),
        code_block: Style::new().fg(Color::Rgb(248, 248, 242)),
        code_border: Style::new().fg(Color::Rgb(98, 114, 164)),
        code_lang_label: Style::new().fg(Color::Rgb(98, 114, 164)).italic(),
        code_line_number: Style::new().fg(Color::Rgb(98, 114, 164)),
        blockquote: Style::new().fg(Color::Rgb(98, 114, 164)).italic(),
        blockquote_bar: Style::new().fg(Color::Rgb(189, 147, 249)),
        link_text: Style::new().fg(Color::Rgb(139, 233, 253)).underline(),
        link_url: Style::new().fg(Color::Rgb(98, 114, 164)),
        list_bullet: Style::new().fg(Color::Rgb(80, 250, 123)),
        list_number: Style::new().fg(Color::Rgb(80, 250, 123)),
        task_done: Style::new().fg(Color::Rgb(80, 250, 123)),
        task_pending: Style::new().fg(Color::Rgb(98, 114, 164)),
        table_border: Style::new().fg(Color::Rgb(98, 114, 164)),
        table_header: Style::new().fg(Color::Rgb(189, 147, 249)).bold(),
        table_cell: Style::new().fg(Color::Rgb(248, 248, 242)),
        thematic_break: Style::new().fg(Color::Rgb(98, 114, 164)),
        image_alt: Style::new().fg(Color::Rgb(255, 121, 198)).italic(),
        footnote_ref: Style::new().fg(Color::Rgb(139, 233, 253)),
        footnote_def: Style::new().fg(Color::Rgb(98, 114, 164)),
    }
}

pub fn monochrome_theme() -> Theme {
    Theme {
        heading1: Style::new().bold(),
        heading2: Style::new().bold(),
        heading3: Style::new().bold(),
        heading4: Style::new().bold().dim(),
        heading5: Style::new().bold().dim(),
        heading6: Style::new().bold().dim(),
        paragraph: Style::new(),
        emphasis: Style::new().italic(),
        strong: Style::new().bold(),
        strikethrough: Style::new().strikethrough(),
        inline_code: Style::new().dim(),
        code_block: Style::new(),
        code_border: Style::new().dim(),
        code_lang_label: Style::new().dim().italic(),
        code_line_number: Style::new().dim(),
        blockquote: Style::new().italic().dim(),
        blockquote_bar: Style::new().dim(),
        link_text: Style::new().underline(),
        link_url: Style::new().dim(),
        list_bullet: Style::new(),
        list_number: Style::new(),
        task_done: Style::new().bold(),
        task_pending: Style::new().dim(),
        table_border: Style::new().dim(),
        table_header: Style::new().bold(),
        table_cell: Style::new(),
        thematic_break: Style::new().dim(),
        image_alt: Style::new().italic(),
        footnote_ref: Style::new().dim(),
        footnote_def: Style::new().dim(),
    }
}

/// Get a built-in theme by name.
pub fn builtin_theme(name: &str) -> Option<Theme> {
    match name {
        "default" => Some(default_theme()),
        "solarized" => Some(solarized_theme()),
        "dracula" => Some(dracula_theme()),
        "monochrome" => Some(monochrome_theme()),
        _ => None,
    }
}

/// List all built-in theme names.
pub fn builtin_theme_names() -> &'static [&'static str] {
    &["default", "solarized", "dracula", "monochrome"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme_has_styled_headings() {
        let t = default_theme();
        assert!(t.heading1.bold);
        assert!(t.heading1.fg.is_some());
        assert!(t.heading2.bold);
        assert!(t.heading3.bold);
    }

    #[test]
    fn test_all_builtin_themes_constructable() {
        for name in builtin_theme_names() {
            let theme = builtin_theme(name);
            assert!(theme.is_some(), "builtin_theme({}) returned None", name);
        }
    }

    #[test]
    fn test_builtin_theme_unknown_returns_none() {
        assert!(builtin_theme("nonexistent").is_none());
    }

    #[test]
    fn test_solarized_uses_rgb_colors() {
        let t = solarized_theme();
        assert!(matches!(t.heading1.fg, Some(Color::Rgb(_, _, _))));
    }

    #[test]
    fn test_dracula_uses_rgb_colors() {
        let t = dracula_theme();
        assert!(matches!(t.heading1.fg, Some(Color::Rgb(_, _, _))));
    }

    #[test]
    fn test_monochrome_no_colors() {
        let t = monochrome_theme();
        assert!(t.heading1.fg.is_none());
        assert!(t.heading1.bold);
        assert!(t.paragraph.fg.is_none());
    }

    #[test]
    fn test_parse_named_colors() {
        assert_eq!(parse_color("red"), Some(Color::Ansi(1)));
        assert_eq!(parse_color("green"), Some(Color::Ansi(2)));
        assert_eq!(parse_color("blue"), Some(Color::Ansi(4)));
        assert_eq!(parse_color("cyan"), Some(Color::Ansi(6)));
        assert_eq!(parse_color("bright_red"), Some(Color::Ansi(9)));
        assert_eq!(parse_color("gray"), Some(Color::Ansi(8)));
        assert_eq!(parse_color("grey"), Some(Color::Ansi(8)));
    }

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_color("#ff0000"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(parse_color("#00ff00"), Some(Color::Rgb(0, 255, 0)));
        assert_eq!(parse_color("#0000ff"), Some(Color::Rgb(0, 0, 255)));
    }

    #[test]
    fn test_parse_palette_index() {
        assert_eq!(parse_color("42"), Some(Color::Palette(42)));
        assert_eq!(parse_color("0"), Some(Color::Palette(0)));
        assert_eq!(parse_color("255"), Some(Color::Palette(255)));
    }

    #[test]
    fn test_parse_invalid_color() {
        assert!(parse_color("notacolor").is_none());
        assert!(parse_color("#gg0000").is_none());
        assert!(parse_color("#fff").is_none());
    }

    #[test]
    fn test_theme_style_def_to_style() {
        let def = ThemeStyleDef {
            fg: Some("red".to_string()),
            bg: Some("#00ff00".to_string()),
            bold: true,
            italic: false,
            underline: true,
            dim: false,
            strikethrough: false,
        };
        let style = def.to_style();
        assert_eq!(style.fg, Some(Color::Ansi(1)));
        assert_eq!(style.bg, Some(Color::Rgb(0, 255, 0)));
        assert!(style.bold);
        assert!(style.underline);
        assert!(!style.italic);
    }

    #[test]
    fn test_overlay_uses_base_when_def_empty() {
        let base = Style::new().bold().fg(Color::Ansi(1));
        let empty_def = ThemeStyleDef::default();
        let result = overlay(&base, &empty_def);
        assert!(result.bold);
        assert_eq!(result.fg, Some(Color::Ansi(1)));
    }

    #[test]
    fn test_overlay_replaces_with_def_when_set() {
        let base = Style::new().bold().fg(Color::Ansi(1));
        let def = ThemeStyleDef {
            fg: Some("blue".to_string()),
            bold: false,
            italic: true,
            ..Default::default()
        };
        let result = overlay(&base, &def);
        assert_eq!(result.fg, Some(Color::Ansi(4)));
        assert!(result.italic);
        assert!(!result.bold);
    }

    #[test]
    fn test_theme_file_toml_round_trip() {
        let toml_str = r##"
[heading1]
fg = "#e06c75"
bold = true

[heading2]
fg = "green"
"##;
        let file: ThemeFile = toml::from_str(toml_str).unwrap();
        let theme = file.into_theme();
        assert!(theme.heading1.bold);
        assert_eq!(theme.heading1.fg, Some(Color::Rgb(224, 108, 117)));
        assert_eq!(theme.heading2.fg, Some(Color::Ansi(2)));
        // Unspecified fields use defaults
        assert!(theme.heading3.bold);
    }

    #[test]
    fn test_theme_default_trait() {
        let t1 = Theme::default();
        let t2 = default_theme();
        assert_eq!(t1.heading1.fg, t2.heading1.fg);
        assert_eq!(t1.heading1.bold, t2.heading1.bold);
    }

    #[test]
    fn test_builtin_theme_names_count() {
        let names = builtin_theme_names();
        assert_eq!(names.len(), 4);
        assert!(names.contains(&"default"));
        assert!(names.contains(&"dracula"));
        assert!(names.contains(&"solarized"));
        assert!(names.contains(&"monochrome"));
    }
}
