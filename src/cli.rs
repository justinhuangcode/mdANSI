use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "mdansi",
    version,
    about = "Render Markdown beautifully in your terminal",
    long_about = "mdANSI — A blazing-fast Markdown-to-ANSI terminal renderer with built-in \
                  syntax highlighting. Supports GFM tables, code blocks, task lists, \
                  hyperlinks, and streaming mode for LLM output.",
    after_help = "EXAMPLES:\n  \
                  mdansi README.md\n  \
                  cat doc.md | mdansi\n  \
                  mdansi --theme dracula --width 120 CHANGELOG.md\n  \
                  curl -s https://raw.githubusercontent.com/.../README.md | mdansi --stream"
)]
pub struct Cli {
    /// Markdown file to render. Reads from stdin if omitted.
    pub file: Option<PathBuf>,

    /// Terminal width (columns). Auto-detected if not specified.
    #[arg(short, long, env = "MDANSI_WIDTH")]
    pub width: Option<usize>,

    /// Color theme.
    #[arg(short, long, default_value = "default", env = "MDANSI_THEME")]
    pub theme: String,

    /// Path to a custom .toml theme file.
    #[arg(long)]
    pub theme_file: Option<PathBuf>,

    /// Disable text wrapping.
    #[arg(long)]
    pub no_wrap: bool,

    /// Disable syntax highlighting.
    #[arg(long)]
    pub no_highlight: bool,

    /// Show line numbers in code blocks.
    #[arg(short = 'n', long)]
    pub line_numbers: bool,

    /// Disable code wrapping inside code blocks.
    #[arg(long)]
    pub no_code_wrap: bool,

    /// Table border style.
    #[arg(long, default_value = "unicode", value_enum)]
    pub table_border: TableBorderArg,

    /// Disable table cell truncation.
    #[arg(long)]
    pub no_truncate: bool,

    /// Force enable/disable colors.
    #[arg(long, value_enum)]
    pub color: Option<ColorArg>,

    /// Streaming mode: render input incrementally (for piped LLM output).
    #[arg(short, long)]
    pub stream: bool,

    /// Output plain text (strip all ANSI codes).
    #[arg(long)]
    pub plain: bool,

    /// List available built-in themes and exit.
    #[arg(long)]
    pub list_themes: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TableBorderArg {
    Unicode,
    Ascii,
    None,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ColorArg {
    Always,
    Never,
    Auto,
}
