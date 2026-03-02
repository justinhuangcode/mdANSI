use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};

/// Parse a Markdown string into a comrak AST.
///
/// Returns the root document node. The arena must outlive all references to the AST.
pub fn parse_markdown<'a>(arena: &'a Arena<AstNode<'a>>, input: &str) -> &'a AstNode<'a> {
    let options = markdown_options();
    parse_document(arena, input, &options)
}

/// Configure comrak with full GFM support + useful extensions.
fn markdown_options() -> Options {
    let mut options = Options::default();

    // Extensions: GFM + extras
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = false;
    options.extension.header_ids = None;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = None;
    options.extension.multiline_block_quotes = false;
    options.extension.math_dollars = true;
    options.extension.math_code = false;
    options.extension.shortcodes = true;
    options.extension.wikilinks_title_after_pipe = false;
    options.extension.wikilinks_title_before_pipe = false;
    options.extension.underline = true;
    options.extension.subscript = false;
    options.extension.spoiler = false;
    options.extension.greentext = false;

    // Parse options
    options.parse.smart = true;
    options.parse.default_info_string = None;
    options.parse.relaxed_tasklist_matching = true;
    options.parse.relaxed_autolinks = true;

    // Render options (we don't use comrak's HTML renderer, but set sensible defaults)
    options.render.hardbreaks = false;
    options.render.github_pre_lang = true;
    options.render.full_info_string = true;
    options.render.width = 0;
    options.render.unsafe_ = true;
    options.render.escape = false;
    options.render.sourcepos = false;
    options.render.prefer_fenced = true;

    options
}

/// Collect all text content from a node and its descendants (no styling).
pub fn collect_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    collect_text_recursive(node, &mut text);
    text
}

fn collect_text_recursive<'a>(node: &'a AstNode<'a>, out: &mut String) {
    match &node.data.borrow().value {
        NodeValue::Text(ref t) => out.push_str(t),
        NodeValue::Code(ref c) => out.push_str(&c.literal),
        NodeValue::SoftBreak => out.push(' '),
        NodeValue::LineBreak => out.push('\n'),
        _ => {
            for child in node.children() {
                collect_text_recursive(child, out);
            }
        }
    }
}

/// Check if a table row is the header row.
pub fn table_row_is_header(node: &AstNode<'_>) -> bool {
    match &node.data.borrow().value {
        NodeValue::TableRow(header) => *header,
        _ => false,
    }
}
