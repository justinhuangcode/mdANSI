//! # mdANSI
//!
//! A blazing-fast Markdown-to-ANSI terminal renderer with built-in syntax highlighting.
//!
//! ## Quick Start
//!
//! ```rust
//! use mdansi::render_markdown;
//!
//! let markdown = "# Hello\n\nThis is **bold** and *italic*.";
//! let ansi = render_markdown(markdown);
//! print!("{}", ansi);
//! ```
//!
//! ## With Custom Options
//!
//! ```rust
//! use mdansi::{Renderer, RenderOptions, Theme};
//!
//! let theme = Theme::default();
//! let options = RenderOptions { width: 100, ..Default::default() };
//! let renderer = Renderer::new(theme, options);
//! let output = renderer.render("## Custom rendering");
//! print!("{}", output);
//! ```
//!
//! ## Streaming (for LLM output)
//!
//! ```rust,no_run
//! use mdansi::{StreamRenderer, RenderOptions, Theme};
//!
//! let stdout = std::io::stdout().lock();
//! let mut stream = StreamRenderer::new(stdout, Theme::default(), RenderOptions::default());
//! stream.push("# Streaming\n").unwrap();
//! stream.push("Hello **world**\n").unwrap();
//! stream.flush_remaining().unwrap();
//! ```

pub mod error;
pub mod highlight;
pub mod hyperlink;
pub mod parser;
pub mod render;
pub mod stream;
pub mod style;
pub mod table;
pub mod terminal;
pub mod theme;
pub mod wrap;

// Re-export primary API for convenience
pub use render::{render_markdown, render_markdown_with_width, RenderOptions, Renderer};
pub use stream::StreamRenderer;
pub use style::{Color, Style};
pub use terminal::TerminalCaps;
pub use theme::Theme;
