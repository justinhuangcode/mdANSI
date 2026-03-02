use thiserror::Error;

/// All errors that can occur in mdANSI.
#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read input: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid theme file: {0}")]
    ThemeParse(#[from] toml::de::Error),

    #[error("syntax highlighting error: {0}")]
    Highlight(#[from] syntect::Error),

    #[error("unknown theme: {name}")]
    UnknownTheme { name: String },

    #[error("terminal width must be at least 20, got {width}")]
    InvalidWidth { width: usize },
}

pub type Result<T> = std::result::Result<T, Error>;
