use mdansi::{render_markdown_with_width, RenderOptions, Renderer, Theme};

#[test]
fn test_basic_heading() {
    let output = render_markdown_with_width("# Hello", 80);
    assert!(output.contains("Hello"));
}

#[test]
fn test_bold_and_italic() {
    let output = render_markdown_with_width("**bold** and *italic*", 80);
    assert!(output.contains("bold"));
    assert!(output.contains("italic"));
}

#[test]
fn test_inline_code() {
    let output = render_markdown_with_width("Use `println!` to print.", 80);
    assert!(output.contains("println!"));
}

#[test]
fn test_code_block() {
    let md = "```rust\nfn main() {}\n```";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("fn main()"));
}

#[test]
fn test_blockquote() {
    let output = render_markdown_with_width("> quoted text", 80);
    assert!(output.contains("quoted text"));
}

#[test]
fn test_unordered_list() {
    let md = "- one\n- two\n- three";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("one"));
    assert!(output.contains("two"));
    assert!(output.contains("three"));
}

#[test]
fn test_ordered_list() {
    let md = "1. first\n2. second";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("first"));
    assert!(output.contains("second"));
}

#[test]
fn test_link() {
    let md = "[example](https://example.com)";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("example"));
}

#[test]
fn test_table() {
    let md = "| Name | Age |\n|------|-----|\n| Alice | 30 |";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("Name"));
    assert!(output.contains("Alice"));
    assert!(output.contains("30"));
}

#[test]
fn test_thematic_break() {
    let md = "above\n\n---\n\nbelow";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("above"));
    assert!(output.contains("below"));
}

#[test]
fn test_empty_input() {
    let output = render_markdown_with_width("", 80);
    assert!(output.is_empty());
}

#[test]
fn test_plain_text_no_ansi() {
    let options = RenderOptions {
        width: 80,
        plain: true,
        highlight: false,
        ..Default::default()
    };
    let renderer = Renderer::new(Theme::default(), options);
    let output = renderer.render("# Hello **world**");
    // Should not contain ANSI escape sequences
    assert!(!output.contains("\x1b["));
}

#[test]
fn test_custom_width() {
    let output = render_markdown_with_width(
        "This is a somewhat long line that should wrap when the terminal width is narrow enough.",
        40,
    );
    // All lines should be within 40 chars (visible width)
    for line in output.lines() {
        let vis = mdansi::wrap::visible_width_of(line);
        assert!(vis <= 40, "Line too wide ({}): {:?}", vis, line);
    }
}

#[test]
fn test_fixture_basic() {
    let md = include_str!("fixtures/basic.md");
    let output = render_markdown_with_width(md, 80);
    // Smoke test: should produce output without panicking
    assert!(!output.is_empty());
    assert!(output.contains("Heading 1"));
    assert!(output.contains("Hello, world!"));
}

// ── Table edge cases ────────────────────────────────────────────────

#[test]
fn test_table_single_column() {
    let md = "| Header |\n|--------|\n| Cell |";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("Header"));
    assert!(output.contains("Cell"));
}

#[test]
fn test_table_many_columns() {
    let md = "| A | B | C | D | E | F |\n|---|---|---|---|---|---|\n| 1 | 2 | 3 | 4 | 5 | 6 |";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("A"));
    assert!(output.contains("F"));
    assert!(output.contains("6"));
}

#[test]
fn test_table_narrow_terminal() {
    let md = "| Name | Description | Status |\n|------|-------------|--------|\n| alpha | A long description here | Active |";
    let output = render_markdown_with_width(md, 40);
    // Should produce output without panicking
    assert!(output.contains("Name"));
}

#[test]
fn test_table_empty_cells() {
    let md = "| A | B |\n|---|---|\n|  |  |";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("A"));
    assert!(output.contains("B"));
}

// ── Nested and complex structures ───────────────────────────────────

#[test]
fn test_nested_blockquote() {
    let md = "> outer\n>> inner";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("outer"));
    assert!(output.contains("inner"));
}

#[test]
fn test_task_list() {
    let md = "- [x] Done\n- [ ] Not done";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("Done"));
    assert!(output.contains("Not done"));
}

#[test]
fn test_strikethrough() {
    let md = "~~deleted~~";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("deleted"));
}

#[test]
fn test_heading_levels() {
    for level in 1..=6 {
        let md = format!("{} Heading {}", "#".repeat(level), level);
        let output = render_markdown_with_width(&md, 80);
        assert!(output.contains(&format!("Heading {}", level)));
    }
}

#[test]
fn test_multiple_code_blocks() {
    let md = "```rust\nfn a() {}\n```\n\nText between.\n\n```python\ndef b(): pass\n```";
    let output = render_markdown_with_width(md, 80);
    assert!(output.contains("fn a()"));
    assert!(output.contains("Text between"));
    assert!(output.contains("def b()"));
}
