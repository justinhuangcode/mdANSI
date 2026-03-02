use unicode_width::UnicodeWidthStr;

/// Wrap text to fit within `max_width` visible columns.
///
/// Features:
/// - ANSI-escape aware (escape sequences don't count toward width)
/// - Unicode-width aware (CJK, emoji handled correctly)
/// - Word-boundary breaking (no mid-word breaks except overflow)
/// - Orphan prevention (avoids trailing short articles)
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    for line in text.split('\n') {
        wrap_line(line, max_width, &mut lines);
    }
    lines
}

fn wrap_line(line: &str, max_width: usize, out: &mut Vec<String>) {
    let stripped = strip_ansi(line);
    let visible_width = UnicodeWidthStr::width(stripped.as_str());

    if visible_width <= max_width {
        out.push(line.to_string());
        return;
    }

    let words = split_words(line);
    let mut current_line = String::new();
    let mut current_width: usize = 0;
    let mut carry_word: Option<String> = None;

    for word in &words {
        let word_vis_width = visible_width_of(word);

        if current_width == 0 {
            // First word on the line — check for carry from orphan prevention
            if let Some(carry) = carry_word.take() {
                current_line.push_str(&carry);
                current_line.push(' ');
                current_line.push_str(word);
                current_width = visible_width_of(&carry) + 1 + word_vis_width;
            } else {
                current_line.push_str(word);
                current_width = word_vis_width;
            }
        } else if current_width + 1 + word_vis_width <= max_width {
            // Fits with a space
            current_line.push(' ');
            current_line.push_str(word);
            current_width += 1 + word_vis_width;
        } else {
            // Doesn't fit — check for orphan and flush
            let (flushed, orphan) = apply_orphan_prevention(current_line, max_width);
            out.push(flushed);
            carry_word = orphan;
            current_line = word.to_string();
            current_width = word_vis_width;
        }
    }

    // Handle final line with any remaining carry
    if let Some(carry) = carry_word.take() {
        if current_line.is_empty() {
            current_line = carry;
        } else {
            // Prepend carry to current line
            current_line = format!("{} {}", carry, current_line);
        }
    }

    if !current_line.is_empty() {
        out.push(current_line);
    }
}

/// Split text into words, preserving ANSI sequences attached to words.
fn split_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut in_escape = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            in_escape = true;
            current.push(ch);
        } else if in_escape {
            current.push(ch);
            // End of escape: letter (a-zA-Z) terminates CSI, BEL terminates OSC
            if ch.is_ascii_alphabetic() || ch == '\x07' {
                in_escape = false;
            }
        } else if ch == ' ' || ch == '\t' {
            if !current.is_empty() {
                let stripped = strip_ansi(&current);
                if !stripped.is_empty() {
                    words.push(current.clone());
                } else {
                    // Pure ANSI — attach to next word
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch != ' ' && next_ch != '\t' {
                            continue; // Keep accumulating
                        }
                    }
                    // Attach trailing ANSI to last word
                    if let Some(last) = words.last_mut() {
                        last.push_str(&current);
                    }
                }
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        let stripped = strip_ansi(&current);
        if !stripped.is_empty() {
            words.push(current);
        } else if let Some(last) = words.last_mut() {
            last.push_str(&current);
        }
    }

    words
}

/// Orphan prevention: if the last word is a short article/preposition,
/// move it to the next line with the previous word.
const ORPHAN_WORDS: &[&str] = &[
    "a", "an", "the", "of", "to", "in", "on", "at", "by", "or", "is", "it", "if", "as", "no", "so",
];

/// Returns (line_without_orphan, Option<removed_orphan_word>).
/// If the last word is a short article/preposition and there are 3+ words,
/// it gets pulled off to join the next line.
fn apply_orphan_prevention(line: String, _max_width: usize) -> (String, Option<String>) {
    let words = split_words(&line);
    if words.len() < 3 {
        return (line, None);
    }

    let last = &words[words.len() - 1];
    let stripped_last = strip_ansi(last).to_lowercase();
    if ORPHAN_WORDS.contains(&stripped_last.as_str()) {
        // Remove last word from this line, return it as carry
        let orphan = last.clone();
        let shortened: String = words[..words.len() - 1].join(" ");
        (shortened, Some(orphan))
    } else {
        (line, None)
    }
}

/// Calculate the visible width of a string, ignoring ANSI escape sequences.
pub fn visible_width_of(s: &str) -> usize {
    let stripped = strip_ansi(s);
    UnicodeWidthStr::width(stripped.as_str())
}

/// Strip ANSI escape sequences from a string.
pub fn strip_ansi(s: &str) -> String {
    let bytes = s.as_bytes();
    let stripped = strip_ansi_escapes::strip(bytes);
    String::from_utf8_lossy(&stripped).into_owned()
}

/// Pad a string to a given visible width with spaces.
pub fn pad_to_width(s: &str, target_width: usize) -> String {
    let current = visible_width_of(s);
    if current >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - current))
    }
}

/// Center a string within a given visible width.
pub fn center_in_width(s: &str, target_width: usize) -> String {
    let current = visible_width_of(s);
    if current >= target_width {
        return s.to_string();
    }
    let total_padding = target_width - current;
    let left = total_padding / 2;
    let right = total_padding - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

/// Right-align a string within a given visible width.
pub fn right_align_in_width(s: &str, target_width: usize) -> String {
    let current = visible_width_of(s);
    if current >= target_width {
        return s.to_string();
    }
    format!("{}{}", " ".repeat(target_width - current), s)
}

/// Truncate a string to fit within max visible width, adding ellipsis if needed.
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let stripped = strip_ansi(s);
    let vis_width = UnicodeWidthStr::width(stripped.as_str());
    if vis_width <= max_width {
        return s.to_string();
    }

    // Walk the stripped string char by char to find the cut point
    let mut width = 0;
    let mut cut_idx = 0;
    for (i, ch) in stripped.char_indices() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width + 1 > max_width {
            // +1 for the ellipsis
            break;
        }
        width += ch_width;
        cut_idx = i + ch.len_utf8();
    }

    format!("{}\u{2026}", &stripped[..cut_idx])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_short_line() {
        let result = wrap_text("hello world", 80);
        assert_eq!(result, vec!["hello world"]);
    }

    #[test]
    fn test_wrap_long_line() {
        let result = wrap_text("the quick brown fox jumps over the lazy dog", 20);
        assert!(result.len() > 1);
        for line in &result {
            assert!(visible_width_of(line) <= 20);
        }
    }

    #[test]
    fn test_visible_width_ansi() {
        let s = "\x1b[1mhello\x1b[0m";
        assert_eq!(visible_width_of(s), 5);
    }

    #[test]
    fn test_truncate() {
        let result = truncate_to_width("hello world", 8);
        assert_eq!(visible_width_of(&result), 8);
    }

    #[test]
    fn test_pad_to_width() {
        let result = pad_to_width("hi", 10);
        assert_eq!(visible_width_of(&result), 10);
    }
}
