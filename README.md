# mdANSI

**English** | [中文](./README_CN.md)

[![CI](https://img.shields.io/github/actions/workflow/status/justinhuangcode/mdANSI/ci.yml?branch=main&label=CI&style=flat-square)](https://github.com/justinhuangcode/mdANSI/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/mdansi?style=flat-square)](https://crates.io/crates/mdansi)
[![docs.rs](https://img.shields.io/docsrs/mdansi?style=flat-square)](https://docs.rs/mdansi)
[![License](https://img.shields.io/crates/l/mdansi?style=flat-square)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?style=flat-square)](https://www.rust-lang.org)
[![GitHub Stars](https://img.shields.io/github/stars/justinhuangcode/mdANSI?style=flat-square)](https://github.com/justinhuangcode/mdANSI/stargazers)
[![Last Commit](https://img.shields.io/github/last-commit/justinhuangcode/mdANSI?style=flat-square)](https://github.com/justinhuangcode/mdANSI/commits/main)
[![Issues](https://img.shields.io/github/issues/justinhuangcode/mdANSI?style=flat-square)](https://github.com/justinhuangcode/mdANSI/issues)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-blue?style=flat-square)]()

A blazing-fast Markdown-to-ANSI terminal renderer with built-in syntax highlighting, streaming mode for LLM output, and TOML-configurable themes. Single binary, zero runtime dependencies.

---

## Why mdANSI?

| Criteria | mdANSI | [Markdansi](https://github.com/nicholasgasior/markdansi) | [mdcat](https://github.com/swsnr/mdcat) | [glow](https://github.com/charmbracelet/glow) |
|----------|--------|-----------|-------|------|
| Language | Rust | TypeScript | Rust | Go |
| Binary size | ~4MB | N/A (Node.js) | ~10MB | ~8MB |
| Runtime deps | None | Node.js 18+ | None | None |
| Syntax highlighting | Built-in (syntect) | External (Shiki) | Built-in (syntect) | Built-in (glamour) |
| Streaming mode | Yes | Yes | No | No |
| Custom themes | TOML files | Code-only | No | Glamour JSON |
| GFM tables | Yes | Yes | Yes | Yes |
| Footnotes | Yes | No | Yes | No |
| Task lists | Yes | Yes | Yes | Yes |
| Math | LaTeX passthrough | No | No | No |
| OSC-8 hyperlinks | Yes | Yes | Yes | No |
| Line numbers | Yes | No | No | No |
| Text wrapping | Unicode + CJK | Basic | Basic | Yes |
| Startup time | ~1ms | ~100ms | ~5ms | ~3ms |

**mdANSI** combines the speed of a compiled Rust binary with the flexibility of a full theme system and the streaming capability that LLM-powered workflows demand.

---

## Features

- **Built-in syntax highlighting** -- 200+ languages via syntect, zero configuration needed
- **Full GFM support** -- tables, task lists, strikethrough, autolinks, footnotes, math (LaTeX passthrough)
- **Streaming mode** -- incremental rendering for piped LLM/AI output with buffered multi-line constructs
- **TOML theme system** -- 4 built-in themes + fully customizable `.toml` theme files
- **Smart text wrapping** -- Unicode-aware, CJK/emoji-correct, orphan prevention
- **OSC-8 hyperlinks** -- clickable terminal links in supported emulators
- **Box-drawn code blocks** -- with language labels and optional line numbers
- **Adaptive terminal detection** -- auto-detects color level, width, and capabilities
- **Single static binary** -- ~4MB, no runtime dependencies, instant startup
- **Dual-mode crate** -- use as a CLI tool or embed as a Rust library

---

## Installation

### Pre-built Binaries (Recommended)

```bash
cargo binstall mdansi
```

### From crates.io

```bash
cargo install mdansi
```

### From Source

```bash
git clone https://github.com/justinhuangcode/mdANSI.git
cd mdANSI
cargo install --path .
```

### Verify Installation

```bash
mdansi --version
```

---

## Quick Start

```bash
# Render a Markdown file
mdansi README.md

# Pipe from stdin
cat CHANGELOG.md | mdansi

# Stream mode for LLM output
llm_command | mdansi --stream

# Custom theme
mdansi --theme dracula doc.md

# With line numbers
mdansi -n README.md
```

---

## Commands

### CLI Options

| Flag | Description | Default |
|------|-------------|---------|
| `[FILE]` | Markdown file to render (stdin if omitted) | -- |
| `-w, --width <N>` | Terminal width override | auto-detected |
| `-t, --theme <NAME>` | Color theme name | `default` |
| `--theme-file <PATH>` | Custom `.toml` theme file | -- |
| `--no-wrap` | Disable text wrapping | off |
| `--no-highlight` | Disable syntax highlighting | off |
| `-n, --line-numbers` | Show line numbers in code blocks | off |
| `--no-code-wrap` | Disable wrapping inside code blocks | off |
| `--table-border <S>` | Table borders: `unicode` / `ascii` / `none` | `unicode` |
| `--no-truncate` | Disable table cell truncation | off |
| `--color <MODE>` | Force color: `always` / `never` / `auto` | `auto` |
| `-s, --stream` | Streaming mode for incremental input | off |
| `--plain` | Strip all ANSI codes (plain text output) | off |
| `--list-themes` | List built-in themes | -- |
| `-h, --help` | Print help | -- |
| `-V, --version` | Print version | -- |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `MDANSI_WIDTH` | Override terminal width |
| `MDANSI_THEME` | Default theme name |
| `NO_COLOR` | Disable all colors ([no-color.org](https://no-color.org)) |
| `FORCE_COLOR` | Force color level: `0`-`3` |

---

## Library Usage

### Basic Rendering

```rust
use mdansi::render_markdown;

let md = "# Hello\n\nThis is **bold** and *italic*.";
let ansi = render_markdown(md);
print!("{}", ansi);
```

### Custom Options

```rust
use mdansi::{Renderer, RenderOptions, Theme, TerminalCaps};
use mdansi::theme;

let caps = TerminalCaps::detect();
let theme = theme::dracula_theme();
let options = RenderOptions {
    width: 100,
    line_numbers: true,
    ..RenderOptions::from_terminal(&caps)
};

let renderer = Renderer::new(theme, options);
let output = renderer.render("## Hello from mdANSI!");
print!("{}", output);
```

### Streaming (LLM Output)

```rust
use mdansi::{StreamRenderer, RenderOptions, Theme};
use std::io;

let stdout = io::stdout().lock();
let mut stream = StreamRenderer::new(stdout, Theme::default(), RenderOptions::default());

// Feed chunks as they arrive from the LLM
stream.push("# Streaming\n").unwrap();
stream.push("This is ").unwrap();
stream.push("**incremental** ").unwrap();
stream.push("output.\n").unwrap();

// Flush remaining buffer when stream ends
stream.flush_remaining().unwrap();
```

---

## Themes

### Built-in Themes

| Theme | Description |
|-------|-------------|
| `default` | Balanced colors for dark terminals |
| `solarized` | Solarized Dark palette |
| `dracula` | Dracula color scheme |
| `monochrome` | Bold/italic/dim only, no colors |

### Custom TOML Theme

Create a `.toml` file with any combination of style overrides:

```toml
# my-theme.toml
[heading1]
fg = "#e06c75"
bold = true

[heading2]
fg = "#98c379"
bold = true

[inline_code]
fg = "#61afef"

[code_border]
fg = "#5c6370"
dim = true

[link_text]
fg = "#c678dd"
underline = true
```

```bash
mdansi --theme-file my-theme.toml README.md
```

**Color formats:** named (`red`, `cyan`, `bright_blue`), hex (`#ff5733`), 256-palette index (`42`).

---

## How It Works

1. **Parse** -- Markdown input is parsed into an AST via [comrak](https://github.com/kivikakk/comrak) (CommonMark + GFM extensions).
2. **Walk** -- The AST is traversed depth-first, converting each node into styled ANSI text segments.
3. **Highlight** -- Fenced code blocks are syntax-highlighted via [syntect](https://github.com/trishume/syntect) with the active theme.
4. **Layout** -- Tables are measured and laid out with Unicode-aware column widths and box-drawing borders.
5. **Wrap** -- Long lines are wrapped at word boundaries, respecting ANSI escape sequences and CJK character widths.
6. **Emit** -- The final ANSI string is written to stdout (or returned as a `String` in library mode).

In **streaming mode**, steps 1-6 run incrementally: single-line content is emitted immediately, while multi-line constructs (code blocks, tables) are buffered until complete.

---

## Architecture

```
                  ┌──────────────┐
   Markdown ───>  │   parser.rs  │  comrak AST
                  └──────┬───────┘
                         │
                  ┌──────▼───────┐
                  │  render.rs   │  AST -> ANSI
                  └──┬───┬───┬──┘
                     │   │   │
          ┌──────────┘   │   └──────────┐
          ▼              ▼              ▼
   ┌────────────┐ ┌────────────┐ ┌────────────┐
   │ highlight  │ │  table.rs  │ │  wrap.rs   │
   │    .rs     │ │            │ │            │
   └────────────┘ └────────────┘ └────────────┘
     syntect        box-drawing    Unicode-aware
     200+ langs     column layout  word wrapping

   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
   │  style.rs    │ │  theme.rs    │ │ hyperlink.rs │
   │  ANSI codes  │ │  TOML themes │ │  OSC-8 links │
   └──────────────┘ └──────────────┘ └──────────────┘

   ┌──────────────┐ ┌──────────────┐
   │  stream.rs   │ │ terminal.rs  │
   │  LLM stream  │ │  capability  │
   │  renderer    │ │  detection   │
   └──────────────┘ └──────────────┘
```

---

## Project Structure

```
mdANSI/
├── src/
│   ├── lib.rs          # Public API and re-exports
│   ├── main.rs         # CLI binary entry point
│   ├── cli.rs          # clap argument definitions
│   ├── parser.rs       # comrak Markdown parsing wrapper
│   ├── render.rs       # Core ANSI rendering engine
│   ├── stream.rs       # Streaming renderer (LLM-friendly)
│   ├── style.rs        # ANSI style/color primitives
│   ├── theme.rs        # Theme system with TOML support
│   ├── table.rs        # GFM table layout engine
│   ├── highlight.rs    # syntect syntax highlighting
│   ├── wrap.rs         # Unicode-aware text wrapping
│   ├── hyperlink.rs    # OSC-8 terminal hyperlinks
│   ├── terminal.rs     # Terminal capability detection
│   └── error.rs        # Error types
├── themes/
│   ├── default.toml    # Default dark theme
│   ├── dracula.toml    # Dracula theme
│   └── solarized.toml  # Solarized Dark theme
├── tests/
│   ├── integration.rs  # Integration test suite
│   └── fixtures/       # Test fixtures
├── benches/
│   └── render.rs       # Criterion benchmarks
├── .github/
│   └── workflows/
│       └── ci.yml      # CI: check, test, clippy, fmt, MSRV, audit
├── Cargo.toml
├── CHANGELOG.md
├── LICENSE-MIT
├── LICENSE-APACHE
└── README.md
```

---

## Benchmarks

Run benchmarks locally:

```bash
cargo bench
```

Typical results on Apple M-series hardware:

| Benchmark | Time |
|-----------|------|
| Full document + syntax highlighting | ~2ms |
| Full document, no highlighting | ~0.3ms |
| Plain text output | ~0.2ms |

---

## Security & Environment

| Concern | Mitigation |
|---------|------------|
| Untrusted Markdown input | comrak sandboxes all parsing; no script execution |
| Stream buffer exhaustion | 10 MB hard limit with automatic flush |
| Theme file loading | TOML deserialization only; no code execution |
| Terminal escape injection | All user content is escaped through the ANSI style layer |
| `NO_COLOR` compliance | Fully supported per [no-color.org](https://no-color.org) |

---

## Troubleshooting

Common issues and solutions are tracked in [GitHub Issues](https://github.com/justinhuangcode/mdANSI/issues).

| Problem | Solution |
|---------|----------|
| No colors in output | Check `NO_COLOR` env var; use `--color always` to force |
| Table columns too narrow | Use `--width` to set wider terminal width, or `--no-truncate` |
| Code block not highlighted | Ensure language is specified after the opening fence (e.g., ` ```rust `) |
| Streaming output garbled | Verify your terminal supports ANSI escape sequences |

---

## Contributing

Contributions are welcome! Please open an issue first for significant changes.

```bash
# Development workflow
cargo build          # Build
cargo test           # Run all tests (68 tests)
cargo clippy         # Lint
cargo fmt --check    # Format check
cargo bench          # Benchmarks
```

See [CHANGELOG.md](./CHANGELOG.md) for release history.

---

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

---

**mdANSI** is maintained by [Justin Huang](https://github.com/justinhuangcode).
