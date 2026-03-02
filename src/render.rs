use comrak::nodes::{AstNode, NodeValue};
use comrak::Arena;

use crate::highlight::Highlighter;
use crate::hyperlink;
use crate::parser;
use crate::style::Style;
use crate::table::{self, Table, TableConfig};
use crate::terminal::TerminalCaps;
use crate::theme::Theme;
use crate::wrap::{visible_width_of, wrap_text};

/// Rendering options.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Maximum line width. 0 = no wrapping.
    pub width: usize,
    /// Enable text wrapping.
    pub wrap: bool,
    /// Enable syntax highlighting for code blocks.
    pub highlight: bool,
    /// Show line numbers in code blocks.
    pub line_numbers: bool,
    /// Wrap code inside code blocks.
    pub code_wrap: bool,
    /// Table border style.
    pub table_border: table::BorderStyle,
    /// Truncate table cells that exceed column width.
    pub table_truncate: bool,
    /// Enable OSC-8 hyperlinks.
    pub hyperlinks: bool,
    /// Strip all ANSI styling (plain text output).
    pub plain: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            width: 80,
            wrap: true,
            highlight: true,
            line_numbers: false,
            code_wrap: true,
            table_border: table::BorderStyle::Unicode,
            table_truncate: true,
            hyperlinks: false,
            plain: false,
        }
    }
}

impl RenderOptions {
    /// Create options from detected terminal capabilities.
    pub fn from_terminal(caps: &TerminalCaps) -> Self {
        Self {
            width: caps.width,
            wrap: true,
            highlight: caps.has_color(),
            line_numbers: false,
            code_wrap: true,
            table_border: table::BorderStyle::Unicode,
            table_truncate: true,
            hyperlinks: caps.hyperlinks,
            plain: !caps.has_color(),
        }
    }
}

/// The main rendering engine.
pub struct Renderer {
    theme: Theme,
    options: RenderOptions,
    highlighter: Option<Highlighter>,
}

impl Renderer {
    pub fn new(theme: Theme, options: RenderOptions) -> Self {
        let highlighter = if options.highlight {
            Some(Highlighter::new())
        } else {
            None
        };
        // When plain mode is enabled, use an empty theme (no ANSI codes)
        let theme = if options.plain { plain_theme() } else { theme };
        Self {
            theme,
            options,
            highlighter,
        }
    }

    /// Render a Markdown string to ANSI-styled text.
    pub fn render(&self, markdown: &str) -> String {
        let arena = Arena::new();
        let root = parser::parse_markdown(&arena, markdown);
        let mut output = String::new();
        self.render_node(root, &mut output, &Context::default());

        // Trim trailing whitespace but keep one final newline
        let trimmed = output.trim_end();
        if trimmed.is_empty() {
            String::new()
        } else {
            format!("{}\n", trimmed)
        }
    }

    /// Render a single AST node, dispatching by type.
    fn render_node<'a>(&self, node: &'a AstNode<'a>, out: &mut String, ctx: &Context) {
        // Hold the borrow for the duration of the match
        let data = node.data.borrow();

        match &data.value {
            NodeValue::Document => {
                drop(data);
                self.render_children(node, out, ctx);
            }

            NodeValue::Paragraph => {
                drop(data);
                let mut inline_buf = String::new();
                self.render_inline_children(node, &mut inline_buf, ctx);

                let styled = if ctx.in_blockquote {
                    self.theme.blockquote.paint(&inline_buf)
                } else {
                    inline_buf
                };

                if self.options.wrap && self.options.width > 0 {
                    let effective_width = self.options.width.saturating_sub(ctx.indent);
                    let lines = wrap_text(&styled, effective_width);
                    for line in lines {
                        out.push_str(&" ".repeat(ctx.indent));
                        out.push_str(&line);
                        out.push('\n');
                    }
                } else {
                    out.push_str(&" ".repeat(ctx.indent));
                    out.push_str(&styled);
                    out.push('\n');
                }
                out.push('\n');
            }

            NodeValue::Heading(heading) => {
                let level = heading.level;
                drop(data);
                let text = parser::collect_text(node);

                let style = match level {
                    1 => &self.theme.heading1,
                    2 => &self.theme.heading2,
                    3 => &self.theme.heading3,
                    4 => &self.theme.heading4,
                    5 => &self.theme.heading5,
                    _ => &self.theme.heading6,
                };

                let prefix = "#".repeat(level as usize);
                let rendered = style.paint(&format!("{} {}", prefix, text));
                out.push('\n');
                out.push_str(&rendered);
                out.push('\n');
                out.push('\n');
            }

            NodeValue::CodeBlock(cb) => {
                let literal = cb.literal.clone();
                let info = cb.info.clone();
                drop(data);
                self.render_code_block(&literal, &info, out, ctx);
            }

            NodeValue::BlockQuote => {
                drop(data);
                let bar = self.theme.blockquote_bar.paint("\u{2502}"); // │
                let child_ctx = Context {
                    in_blockquote: true,
                    indent: ctx.indent,
                    ..ctx.clone()
                };

                let mut inner = String::new();
                self.render_children(node, &mut inner, &child_ctx);

                for line in inner.lines() {
                    out.push_str(&" ".repeat(ctx.indent));
                    out.push_str(&bar);
                    out.push(' ');
                    out.push_str(line);
                    out.push('\n');
                }
                out.push('\n');
            }

            NodeValue::List(list) => {
                let is_ordered = list.list_type == comrak::nodes::ListType::Ordered;
                let start = list.start;
                drop(data);

                let mut index = start;
                for child in node.children() {
                    self.render_list_item(child, is_ordered, index, out, ctx);
                    if is_ordered {
                        index += 1;
                    }
                }
                if ctx.list_depth == 0 {
                    out.push('\n');
                }
            }

            NodeValue::Item(_) => {
                drop(data);
                self.render_children(node, out, ctx);
            }

            NodeValue::ThematicBreak => {
                drop(data);
                let width = self.options.width.saturating_sub(ctx.indent);
                let rule = "\u{2500}".repeat(width.min(40)); // ─
                out.push_str(&" ".repeat(ctx.indent));
                out.push_str(&self.theme.thematic_break.paint(&rule));
                out.push('\n');
                out.push('\n');
            }

            NodeValue::Table(ref node_table) => {
                let alignments = node_table.alignments.clone();
                drop(data);
                self.render_table(node, &alignments, out, ctx);
            }

            NodeValue::TableRow(_) | NodeValue::TableCell => {
                // Handled by render_table
            }

            NodeValue::FootnoteDefinition(ref fd) => {
                let name = fd.name.clone();
                drop(data);
                let style = &self.theme.footnote_def;
                out.push_str(&style.paint(&format!("[^{}]: ", name)));
                let mut inner = String::new();
                self.render_children(node, &mut inner, ctx);
                out.push_str(inner.trim());
                out.push('\n');
            }

            NodeValue::HtmlBlock(ref hb) => {
                let literal = hb.literal.clone();
                drop(data);
                let style = Style::new().dim();
                for line in literal.lines() {
                    out.push_str(&style.paint(line));
                    out.push('\n');
                }
            }

            NodeValue::TaskItem(_) => {
                // Handled inline by render_list_item
            }

            // Inline nodes — delegate to inline renderer
            NodeValue::Text(_)
            | NodeValue::Code(_)
            | NodeValue::Emph
            | NodeValue::Strong
            | NodeValue::Strikethrough
            | NodeValue::Link(_)
            | NodeValue::Image(_)
            | NodeValue::SoftBreak
            | NodeValue::LineBreak
            | NodeValue::FootnoteReference(_)
            | NodeValue::Math(_)
            | NodeValue::HtmlInline(_) => {
                drop(data);
                let mut buf = String::new();
                self.render_inline(node, &mut buf, ctx);
                out.push_str(&buf);
            }

            // ShortCode support (comrak shortcodes feature)
            NodeValue::ShortCode(_) => {
                drop(data);
                let mut buf = String::new();
                self.render_inline(node, &mut buf, ctx);
                out.push_str(&buf);
            }

            _ => {
                drop(data);
                self.render_children(node, out, ctx);
            }
        }
    }

    fn render_children<'a>(&self, node: &'a AstNode<'a>, out: &mut String, ctx: &Context) {
        for child in node.children() {
            self.render_node(child, out, ctx);
        }
    }

    /// Render inline content (text, emphasis, code, links, etc.)
    fn render_inline<'a>(&self, node: &'a AstNode<'a>, out: &mut String, ctx: &Context) {
        let data = node.data.borrow();

        match &data.value {
            NodeValue::Text(ref text) => {
                out.push_str(text);
            }

            NodeValue::Code(ref code) => {
                let styled = self.theme.inline_code.paint(&format!("`{}`", code.literal));
                out.push_str(&styled);
            }

            NodeValue::Emph => {
                drop(data);
                let mut inner = String::new();
                self.render_inline_children(node, &mut inner, ctx);
                out.push_str(&self.theme.emphasis.paint(&inner));
            }

            NodeValue::Strong => {
                drop(data);
                let mut inner = String::new();
                self.render_inline_children(node, &mut inner, ctx);
                out.push_str(&self.theme.strong.paint(&inner));
            }

            NodeValue::Strikethrough => {
                drop(data);
                let mut inner = String::new();
                self.render_inline_children(node, &mut inner, ctx);
                out.push_str(&self.theme.strikethrough.paint(&inner));
            }

            NodeValue::Link(ref link) => {
                let url = link.url.clone();
                drop(data);
                let mut text_buf = String::new();
                self.render_inline_children(node, &mut text_buf, ctx);

                let rendered = if self.options.hyperlinks {
                    let styled_text = self.theme.link_text.paint(&text_buf);
                    hyperlink::render_hyperlink(&styled_text, &url, true)
                } else if hyperlink::is_autolink(&text_buf, &url) {
                    self.theme.link_text.paint(&text_buf)
                } else {
                    let text_styled = self.theme.link_text.paint(&text_buf);
                    let url_styled = self.theme.link_url.paint(&format!("({})", url));
                    format!("{} {}", text_styled, url_styled)
                };
                out.push_str(&rendered);
            }

            NodeValue::Image(ref img) => {
                let url = img.url.clone();
                drop(data);
                let alt = parser::collect_text(node);
                let display = if alt.is_empty() {
                    format!("[image: {}]", url)
                } else {
                    format!("[image: {}]", alt)
                };
                out.push_str(&self.theme.image_alt.paint(&display));
            }

            NodeValue::SoftBreak => {
                out.push(' ');
            }

            NodeValue::LineBreak => {
                out.push('\n');
            }

            NodeValue::FootnoteReference(ref fr) => {
                let styled = self.theme.footnote_ref.paint(&format!("[^{}]", fr.name));
                out.push_str(&styled);
            }

            NodeValue::Math(ref math) => {
                let delim = if math.display_math { "$$" } else { "$" };
                let styled = self
                    .theme
                    .inline_code
                    .paint(&format!("{}{}{}", delim, math.literal, delim));
                out.push_str(&styled);
            }

            NodeValue::HtmlInline(ref html) => {
                out.push_str(&Style::new().dim().paint(html));
            }

            // ShortCode support (comrak shortcodes feature)
            NodeValue::ShortCode(ref sc) => {
                out.push_str(&format!(":{}:", sc.code));
            }

            _ => {
                drop(data);
                self.render_inline_children(node, out, ctx);
            }
        }
    }

    fn render_inline_children<'a>(&self, node: &'a AstNode<'a>, out: &mut String, ctx: &Context) {
        for child in node.children() {
            self.render_inline(child, out, ctx);
        }
    }

    /// Render a code block with optional syntax highlighting and box drawing.
    fn render_code_block(&self, literal: &str, info: &str, out: &mut String, ctx: &Context) {
        let lang = info.split_whitespace().next().unwrap_or("").to_string();

        let code = literal.trim_end_matches('\n');
        let lines: Vec<&str> = code.lines().collect();
        let is_single_line = lines.len() == 1;

        // Try syntax highlighting
        let highlighted_lines: Option<Vec<String>> = if !lang.is_empty() {
            self.highlighter
                .as_ref()
                .and_then(|h| h.highlight_to_ansi(code, &lang))
        } else {
            None
        };

        let display_lines: Vec<String> = match highlighted_lines {
            Some(hl) => hl,
            None => lines
                .iter()
                .map(|l| self.theme.code_block.paint(l))
                .collect(),
        };

        let indent = ctx.indent;
        let indent_str = " ".repeat(indent);
        let effective_width = self.options.width.saturating_sub(indent);

        if is_single_line {
            let line = display_lines.first().map(|s| s.as_str()).unwrap_or("");
            out.push_str(&indent_str);
            out.push_str(&self.theme.code_border.paint("\u{2502} "));
            out.push_str(line);
            out.push('\n');
            out.push('\n');
            return;
        }

        // Multi-line: render with box
        let gutter_width = if self.options.line_numbers {
            let max_num = display_lines.len();
            format!("{}", max_num).len() + 1
        } else {
            0
        };

        let border_chars = 4; // "│ " + " │"
        let content_width = effective_width.saturating_sub(border_chars + gutter_width);

        // Top border with language label
        let lang_label = if !lang.is_empty() {
            self.theme.code_lang_label.paint(&format!(" {} ", lang))
        } else {
            String::new()
        };
        let label_vis_width = visible_width_of(&lang_label);
        let top_bar_width = (content_width + gutter_width).saturating_sub(label_vis_width);

        out.push_str(&self.theme.code_border.paint(&format!(
            "{}\u{256d}{}{}\u{256e}",
            indent_str,
            lang_label,
            "\u{2500}".repeat(top_bar_width),
        )));
        out.push('\n');

        // Code lines
        for (i, line) in display_lines.iter().enumerate() {
            let gutter = if self.options.line_numbers {
                self.theme.code_line_number.paint(&format!(
                    "{:>width$} ",
                    i + 1,
                    width = gutter_width - 1
                ))
            } else {
                String::new()
            };

            if self.options.code_wrap && content_width > 0 {
                let wrapped = wrap_text(line, content_width);
                for (j, wline) in wrapped.iter().enumerate() {
                    let g = if j == 0 {
                        gutter.clone()
                    } else {
                        " ".repeat(gutter_width)
                    };
                    out.push_str(
                        &self
                            .theme
                            .code_border
                            .paint(&format!("{}\u{2502} ", indent_str)),
                    );
                    out.push_str(&g);
                    out.push_str(wline);
                    let used = gutter_width + visible_width_of(wline);
                    if used < content_width + gutter_width {
                        out.push_str(&" ".repeat(content_width + gutter_width - used));
                    }
                    out.push_str(&self.theme.code_border.paint(" \u{2502}"));
                    out.push('\n');
                }
            } else {
                out.push_str(
                    &self
                        .theme
                        .code_border
                        .paint(&format!("{}\u{2502} ", indent_str)),
                );
                out.push_str(&gutter);
                out.push_str(line);
                out.push_str(&self.theme.code_border.paint(" \u{2502}"));
                out.push('\n');
            }
        }

        // Bottom border
        let bottom_width = content_width + gutter_width;
        out.push_str(&self.theme.code_border.paint(&format!(
            "{}\u{2570}{}\u{256f}",
            indent_str,
            "\u{2500}".repeat(bottom_width),
        )));
        out.push('\n');
        out.push('\n');
    }

    /// Render a list item with bullet/number and optional task checkbox.
    fn render_list_item<'a>(
        &self,
        node: &'a AstNode<'a>,
        is_ordered: bool,
        index: usize,
        out: &mut String,
        ctx: &Context,
    ) {
        let indent = ctx.indent;
        let indent_str = " ".repeat(indent);

        let prefix = if is_ordered {
            self.theme.list_number.paint(&format!("{}.", index))
        } else {
            self.theme.list_bullet.paint("\u{2022}") // bullet
        };

        // Check for task item
        let task_prefix = self.find_task_prefix(node);
        let full_prefix = match task_prefix {
            Some(true) => format!("{} {}", prefix, self.theme.task_done.paint("[x]")),
            Some(false) => format!("{} {}", prefix, self.theme.task_pending.paint("[ ]")),
            None => prefix,
        };

        let prefix_width = visible_width_of(&full_prefix) + 1;
        let child_ctx = Context {
            indent: indent + prefix_width,
            list_depth: ctx.list_depth + 1,
            ..ctx.clone()
        };

        let mut first_para = true;

        for child in node.children() {
            let is_task_item = matches!(child.data.borrow().value, NodeValue::TaskItem(_));
            if is_task_item {
                continue;
            }

            let is_paragraph = matches!(child.data.borrow().value, NodeValue::Paragraph);

            if first_para {
                if is_paragraph {
                    let mut para_text = String::new();
                    self.render_inline_children(child, &mut para_text, &child_ctx);

                    if self.options.wrap && self.options.width > 0 {
                        let effective_width =
                            self.options.width.saturating_sub(indent + prefix_width);
                        let lines = wrap_text(&para_text, effective_width);
                        for (i, line) in lines.iter().enumerate() {
                            if i == 0 {
                                out.push_str(&indent_str);
                                out.push_str(&full_prefix);
                                out.push(' ');
                                out.push_str(line);
                            } else {
                                out.push_str(&" ".repeat(indent + prefix_width));
                                out.push_str(line);
                            }
                            out.push('\n');
                        }
                    } else {
                        out.push_str(&indent_str);
                        out.push_str(&full_prefix);
                        out.push(' ');
                        out.push_str(&para_text);
                        out.push('\n');
                    }
                    first_para = false;
                } else {
                    let mut inner = String::new();
                    out.push_str(&indent_str);
                    out.push_str(&full_prefix);
                    out.push(' ');
                    self.render_node(child, &mut inner, &child_ctx);
                    out.push_str(inner.trim_start());
                    first_para = false;
                }
            } else {
                self.render_node(child, out, &child_ctx);
            }
        }
    }

    fn find_task_prefix<'a>(&self, node: &'a AstNode<'a>) -> Option<bool> {
        for child in node.children() {
            let data = child.data.borrow();
            if let NodeValue::TaskItem(ref checked) = data.value {
                return Some(checked.is_some());
            }
        }
        None
    }

    /// Render a GFM table.
    fn render_table<'a>(
        &self,
        node: &'a AstNode<'a>,
        alignments: &[comrak::nodes::TableAlignment],
        out: &mut String,
        ctx: &Context,
    ) {
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        for child in node.children() {
            let is_header = parser::table_row_is_header(child);
            let mut cells = Vec::new();

            for cell in child.children() {
                let text = parser::collect_text(cell);
                cells.push(text);
            }

            if is_header {
                headers = cells;
            } else {
                rows.push(cells);
            }
        }

        let table = Table::new(headers, rows, alignments.to_vec());
        let config = TableConfig {
            max_width: self.options.width.saturating_sub(ctx.indent),
            border_style: self.options.table_border,
            border_style_ansi: self.theme.table_border.clone(),
            header_style: self.theme.table_header.clone(),
            cell_style: self.theme.table_cell.clone(),
            truncate: self.options.table_truncate,
            ..TableConfig::default()
        };

        let indent_str = " ".repeat(ctx.indent);
        for line in table.render(&config) {
            out.push_str(&indent_str);
            out.push_str(&line);
            out.push('\n');
        }
        out.push('\n');
    }
}

/// Context passed down the rendering tree.
#[derive(Debug, Clone, Default)]
struct Context {
    indent: usize,
    in_blockquote: bool,
    list_depth: usize,
}

// ──────────────────────────────────────────────────────────────────────────────
// Convenience functions
// ──────────────────────────────────────────────────────────────────────────────

/// Render Markdown to ANSI with default settings.
pub fn render_markdown(markdown: &str) -> String {
    let caps = TerminalCaps::detect();
    let options = RenderOptions::from_terminal(&caps);
    let renderer = Renderer::new(Theme::default(), options);
    renderer.render(markdown)
}

/// Render Markdown to ANSI with a specific width.
pub fn render_markdown_with_width(markdown: &str, width: usize) -> String {
    let caps = TerminalCaps::detect().with_width(width);
    let options = RenderOptions::from_terminal(&caps);
    let renderer = Renderer::new(Theme::default(), options);
    renderer.render(markdown)
}

/// A theme with no ANSI styling at all (for plain text output).
fn plain_theme() -> Theme {
    use crate::style::Style;
    Theme {
        heading1: Style::new(),
        heading2: Style::new(),
        heading3: Style::new(),
        heading4: Style::new(),
        heading5: Style::new(),
        heading6: Style::new(),
        paragraph: Style::new(),
        emphasis: Style::new(),
        strong: Style::new(),
        strikethrough: Style::new(),
        inline_code: Style::new(),
        code_block: Style::new(),
        code_border: Style::new(),
        code_lang_label: Style::new(),
        code_line_number: Style::new(),
        blockquote: Style::new(),
        blockquote_bar: Style::new(),
        link_text: Style::new(),
        link_url: Style::new(),
        list_bullet: Style::new(),
        list_number: Style::new(),
        task_done: Style::new(),
        task_pending: Style::new(),
        table_border: Style::new(),
        table_header: Style::new(),
        table_cell: Style::new(),
        thematic_break: Style::new(),
        image_alt: Style::new(),
        footnote_ref: Style::new(),
        footnote_def: Style::new(),
    }
}
