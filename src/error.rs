use lyon::extra::parser::ParseError;
use thiserror::Error;

pub type Result<T, E = PathError> = std::result::Result<T, E>;

/// Represents all possible errors that can occur within the new_path module.
#[derive(Error, Debug)]
pub enum PathError {
    /// An error occurred while parsing an SVG path data string.
    /// This variant automatically converts from `lyon::extra::parser::ParseError`.
    #[error("Failed to parse SVG path data: {0}")]
    Parse(#[from] ParseError),

    #[error("Failed to fit curve")]
    FitCurve,

    #[error("Failed to clean path")]
    CleanPath,

    /// (推荐添加) An I/O error occurred.
    /// Useful if you ever read paths from files.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
