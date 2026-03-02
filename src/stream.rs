use std::io::Write;

use crate::render::{RenderOptions, Renderer};
use crate::theme::Theme;

// Maximum buffer size (10 MB) to prevent unbounded memory growth.
const MAX_BUFFER_SIZE: usize = 10 * 1024 * 1024;

/// Streaming Markdown renderer for incremental input (e.g., LLM output).
///
/// Design philosophy:
/// - Single-line content is emitted immediately (low latency).
/// - Multi-line constructs (code blocks, tables) are buffered until complete.
/// - Append-only output: no cursor movement, safe for scrollback.
/// - Flush semantics: call `flush()` at end of stream to render remaining buffer.
pub struct StreamRenderer<W: Write> {
    writer: W,
    renderer: Renderer,
    /// Incoming text that hasn't been split into complete lines yet.
    input_buf: String,
    /// Accumulator for multi-line constructs (code blocks, tables).
    accum: String,
    state: StreamState,
    fence_char: char,
    fence_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum StreamState {
    /// Normal text mode: emit lines as they come.
    Normal,
    /// Inside a fenced code block: buffer until closing fence.
    FencedCode,
    /// Inside a table: buffer until non-table line.
    Table,
}

impl<W: Write> StreamRenderer<W> {
    pub fn new(writer: W, theme: Theme, options: RenderOptions) -> Self {
        let renderer = Renderer::new(theme, options);
        Self {
            writer,
            renderer,
            input_buf: String::new(),
            accum: String::new(),
            state: StreamState::Normal,
            fence_char: '`',
            fence_count: 0,
        }
    }

    /// Feed a chunk of text into the stream.
    /// May produce immediate output for single-line content.
    /// Returns an error if the internal buffer exceeds the safety limit.
    pub fn push(&mut self, chunk: &str) -> std::io::Result<()> {
        self.input_buf.push_str(chunk);

        // Safety: flush if buffers grow beyond limits (e.g., unclosed code fence)
        if self.input_buf.len() + self.accum.len() > MAX_BUFFER_SIZE {
            self.flush_remaining()?;
        }

        self.process_lines()
    }

    /// Flush any remaining buffered content.
    /// Call this when the input stream is complete.
    pub fn flush_remaining(&mut self) -> std::io::Result<()> {
        // Flush any accumulated multi-line content first
        if !self.accum.is_empty() {
            let md = std::mem::take(&mut self.accum);
            let rendered = self.renderer.render(&md);
            self.writer.write_all(rendered.as_bytes())?;
        }
        // Then flush any remaining partial line in input_buf
        if !self.input_buf.is_empty() {
            let md = std::mem::take(&mut self.input_buf);
            let rendered = self.renderer.render(&md);
            self.writer.write_all(rendered.as_bytes())?;
        }
        self.writer.flush()?;
        self.state = StreamState::Normal;
        Ok(())
    }

    /// Get a reference to the underlying writer.
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Consume the stream renderer and return the underlying writer.
    pub fn into_writer(self) -> W {
        self.writer
    }

    fn process_lines(&mut self) -> std::io::Result<()> {
        // Extract complete lines from input_buf and dispatch them
        while let Some(newline_pos) = self.input_buf.find('\n') {
            let line = self.input_buf[..newline_pos].to_string();
            self.input_buf = self.input_buf[newline_pos + 1..].to_string();

            match self.state {
                StreamState::Normal => self.handle_normal_line(&line)?,
                StreamState::FencedCode => self.handle_fenced_line(&line)?,
                StreamState::Table => self.handle_table_line(&line)?,
            }
        }
        Ok(())
    }

    fn handle_normal_line(&mut self, line: &str) -> std::io::Result<()> {
        let trimmed = line.trim();

        // Detect start of fenced code block
        if let Some((ch, count, _info)) = detect_fence_open(trimmed) {
            self.state = StreamState::FencedCode;
            self.fence_char = ch;
            self.fence_count = count;
            self.accum.clear();
            self.accum.push_str(line);
            self.accum.push('\n');
            return Ok(());
        }

        // Detect start of table (line with pipes)
        if is_table_line(trimmed) {
            self.state = StreamState::Table;
            self.accum.clear();
            self.accum.push_str(line);
            self.accum.push('\n');
            return Ok(());
        }

        // Single-line content: render and emit immediately
        let md = format!("{}\n", line);
        let rendered = self.renderer.render(&md);
        let output = rendered.trim_end_matches('\n');
        if !output.is_empty() {
            self.writer.write_all(output.as_bytes())?;
            self.writer.write_all(b"\n")?;
            self.writer.flush()?;
        } else if line.is_empty() {
            self.writer.write_all(b"\n")?;
            self.writer.flush()?;
        }
        Ok(())
    }

    fn handle_fenced_line(&mut self, line: &str) -> std::io::Result<()> {
        self.accum.push_str(line);
        self.accum.push('\n');

        // Check if this line closes the fence
        let trimmed = line.trim();
        if is_closing_fence(trimmed, self.fence_char, self.fence_count) {
            let md = std::mem::take(&mut self.accum);
            let rendered = self.renderer.render(&md);
            self.writer.write_all(rendered.as_bytes())?;
            self.writer.flush()?;
            self.state = StreamState::Normal;
        }

        Ok(())
    }

    fn handle_table_line(&mut self, line: &str) -> std::io::Result<()> {
        let trimmed = line.trim();

        if is_table_line(trimmed) || is_table_separator(trimmed) {
            self.accum.push_str(line);
            self.accum.push('\n');
        } else {
            // Table ended — render accumulated table
            let md = std::mem::take(&mut self.accum);
            let rendered = self.renderer.render(&md);
            self.writer.write_all(rendered.as_bytes())?;
            self.writer.flush()?;
            self.state = StreamState::Normal;

            // Process the current (non-table) line normally
            self.handle_normal_line(line)?;
        }
        Ok(())
    }
}

/// Detect a fence opening: returns (fence_char, count, info_string).
fn detect_fence_open(line: &str) -> Option<(char, usize, String)> {
    let trimmed = line.trim_start();
    let first_char = trimmed.chars().next()?;
    if first_char != '`' && first_char != '~' {
        return None;
    }

    let count = trimmed.chars().take_while(|&c| c == first_char).count();
    if count < 3 {
        return None;
    }

    let info = trimmed[count..].trim().to_string();
    Some((first_char, count, info))
}

/// Check if a line is a closing fence.
fn is_closing_fence(line: &str, fence_char: char, min_count: usize) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }

    let first = trimmed.chars().next().unwrap_or(' ');
    if first != fence_char {
        return false;
    }

    let count = trimmed.chars().take_while(|&c| c == fence_char).count();
    count >= min_count && trimmed.chars().skip(count).all(|c| c.is_whitespace())
}

/// Check if a line looks like a table row (contains pipes).
fn is_table_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|') && trimmed.len() > 2
}

/// Check if a line is a table separator (e.g., |---|---|).
fn is_table_separator(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return false;
    }
    trimmed
        .chars()
        .all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::RenderOptions;
    use crate::theme::Theme;

    fn stream_to_string(chunks: &[&str]) -> String {
        let mut buf = Vec::new();
        {
            let mut sr = StreamRenderer::new(
                &mut buf,
                Theme::default(),
                RenderOptions {
                    width: 80,
                    ..Default::default()
                },
            );
            for chunk in chunks {
                sr.push(chunk).unwrap();
            }
            sr.flush_remaining().unwrap();
        }
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn test_stream_plain_text() {
        let output = stream_to_string(&["Hello world\n"]);
        assert!(output.contains("Hello world"));
    }

    #[test]
    fn test_stream_heading() {
        let output = stream_to_string(&["# Title\n"]);
        assert!(output.contains("Title"));
    }

    #[test]
    fn test_stream_incremental_chunks() {
        let output = stream_to_string(&["# He", "llo\n", "World\n"]);
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
    }

    #[test]
    fn test_stream_code_block() {
        let output = stream_to_string(&["```rust\n", "fn main() {}\n", "```\n"]);
        // Syntax highlighting splits tokens, so check individually
        assert!(output.contains("fn") && output.contains("main"));
    }

    #[test]
    fn test_stream_code_block_incremental() {
        // Code block split across multiple pushes
        let output = stream_to_string(&["```\n", "line 1\n", "line 2\n", "```\n"]);
        assert!(output.contains("line 1"));
        assert!(output.contains("line 2"));
    }

    #[test]
    fn test_stream_table() {
        let output = stream_to_string(&["| A | B |\n", "|---|---|\n", "| 1 | 2 |\n", "\n"]);
        assert!(output.contains("A"));
        assert!(output.contains("1"));
    }

    #[test]
    fn test_stream_mixed_content() {
        let output = stream_to_string(&[
            "# Title\n",
            "\n",
            "Paragraph text.\n",
            "\n",
            "```\n",
            "code\n",
            "```\n",
        ]);
        assert!(output.contains("Title"));
        assert!(output.contains("Paragraph"));
        assert!(output.contains("code"));
    }

    #[test]
    fn test_stream_empty_flush() {
        // Flush with nothing buffered should succeed without error
        let mut buf = Vec::new();
        let mut sr = StreamRenderer::new(&mut buf, Theme::default(), RenderOptions::default());
        sr.flush_remaining().unwrap();
        assert!(buf.is_empty());
    }

    #[test]
    fn test_stream_incomplete_flush() {
        // Content without trailing newline should be flushed
        let output = stream_to_string(&["Hello without newline"]);
        assert!(output.contains("Hello without newline"));
    }

    #[test]
    fn test_stream_tilde_fence() {
        let output = stream_to_string(&["~~~\n", "tilde block\n", "~~~\n"]);
        assert!(output.contains("tilde block"));
    }

    // Helper function tests
    #[test]
    fn test_detect_fence_open_backtick() {
        let result = detect_fence_open("```rust");
        assert!(result.is_some());
        let (ch, count, info) = result.unwrap();
        assert_eq!(ch, '`');
        assert_eq!(count, 3);
        assert_eq!(info, "rust");
    }

    #[test]
    fn test_detect_fence_open_tilde() {
        let result = detect_fence_open("~~~python");
        assert!(result.is_some());
        let (ch, count, info) = result.unwrap();
        assert_eq!(ch, '~');
        assert_eq!(count, 3);
        assert_eq!(info, "python");
    }

    #[test]
    fn test_detect_fence_open_too_short() {
        assert!(detect_fence_open("``not a fence").is_none());
        assert!(detect_fence_open("~~not a fence").is_none());
    }

    #[test]
    fn test_detect_fence_open_plain() {
        assert!(detect_fence_open("just text").is_none());
    }

    #[test]
    fn test_is_closing_fence() {
        assert!(is_closing_fence("```", '`', 3));
        assert!(is_closing_fence("````", '`', 3));
        assert!(!is_closing_fence("``", '`', 3));
        assert!(!is_closing_fence("~~~", '`', 3));
        assert!(is_closing_fence("~~~", '~', 3));
    }

    #[test]
    fn test_is_table_line() {
        assert!(is_table_line("| A | B |"));
        assert!(!is_table_line("not a table"));
        assert!(is_table_line("| |")); // len 3 which is > 2, starts/ends with |
        assert!(!is_table_line("| start but no end"));
        assert!(!is_table_line("||")); // len 2, not > 2
    }

    #[test]
    fn test_is_table_separator() {
        assert!(is_table_separator("|---|---|"));
        assert!(is_table_separator("| --- | :---: |"));
        assert!(!is_table_separator("not a separator"));
        assert!(!is_table_separator("| text | here |"));
    }

    #[test]
    fn test_stream_into_writer() {
        let buf = Vec::new();
        let sr = StreamRenderer::new(buf, Theme::default(), RenderOptions::default());
        let recovered = sr.into_writer();
        assert!(recovered.is_empty());
    }
}
