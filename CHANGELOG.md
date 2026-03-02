# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-02

### Added

- **Core rendering engine** -- Full GFM Markdown to ANSI terminal output via comrak AST traversal
- **Built-in syntax highlighting** -- 200+ languages via syntect, zero configuration
- **Streaming renderer** -- Incremental rendering for LLM/AI piped output with state machine (Normal/FencedCode/Table)
- **TOML theme system** -- 4 built-in themes (default, solarized, dracula, monochrome) + custom `.toml` theme files with overlay merging
- **GFM table engine** -- Unicode box-drawing borders, ASCII borders, or borderless; column alignment (left/center/right); adaptive column width with iterative shrinking
- **Smart text wrapping** -- Unicode-aware, CJK/emoji-correct, ANSI-escape preserving, orphan prevention for short articles/prepositions
- **OSC-8 terminal hyperlinks** -- Clickable links in supported terminals with fallback to `text (url)` format
- **Terminal capability detection** -- Auto-detects color level (None/Basic/256/TrueColor), width, hyperlink support, and graphics protocols; respects `NO_COLOR` and `FORCE_COLOR`
- **CLI binary** -- 18 options including `--stream`, `--theme`, `--theme-file`, `--width`, `--line-numbers`, `--plain`, `--no-wrap`, `--table-border`, `--color`, `--list-themes`
- **Library API** -- `render_markdown()`, `render_markdown_with_width()`, `Renderer`, `StreamRenderer` with full doc-tests
- **Plain text mode** -- Strip all ANSI codes for piping to files or non-terminal consumers
- **Box-drawn code blocks** -- With language labels and optional line numbers
- **Criterion benchmarks** -- Full document, no-highlight, and plain text render benchmarks
- **CI pipeline** -- Multi-platform (Linux/macOS/Windows), clippy, fmt, MSRV 1.75 verification, security audit
- **68 tests** -- 42 unit tests, 23 integration tests, 3 doc-tests

[0.1.0]: https://github.com/justinhuangcode/mdANSI/releases/tag/v0.1.0
