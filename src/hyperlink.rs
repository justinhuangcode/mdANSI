/// Render an OSC-8 terminal hyperlink.
///
/// Format: `ESC ] 8 ; params ; uri ST text ESC ] 8 ; ; ST`
/// Where ST (String Terminator) can be `ESC \` or `BEL (\x07)`.
///
/// Falls back to `text (url)` if the terminal doesn't support hyperlinks.
pub fn render_hyperlink(text: &str, url: &str, supports_hyperlinks: bool) -> String {
    if supports_hyperlinks {
        format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
    } else if text == url {
        text.to_string()
    } else {
        format!("{} ({})", text, url)
    }
}

/// Detect if a URL is an autolink (standalone URL with no custom text).
pub fn is_autolink(text: &str, url: &str) -> bool {
    text == url || text == url.strip_prefix("mailto:").unwrap_or(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperlink_with_support() {
        let result = render_hyperlink("click here", "https://example.com", true);
        assert!(result.contains("https://example.com"));
        assert!(result.contains("click here"));
        assert!(result.starts_with("\x1b]8;;"));
    }

    #[test]
    fn test_hyperlink_fallback() {
        let result = render_hyperlink("click here", "https://example.com", false);
        assert_eq!(result, "click here (https://example.com)");
    }

    #[test]
    fn test_hyperlink_same_text_url() {
        let result = render_hyperlink("https://example.com", "https://example.com", false);
        assert_eq!(result, "https://example.com");
    }
}
