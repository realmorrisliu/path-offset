//! Defines the error types used throughout the library.

use lyon::extra::parser::ParseError;
use thiserror::Error;

/// A convenient result type alias for operations within this crate.
pub type Result<T, E = PathError> = std::result::Result<T, E>;

/// Represents all possible errors that can occur within the path processing modules.
#[derive(Error, Debug)]
pub enum PathError {
    /// An error that occurred while parsing an SVG path data string.
    /// This variant automatically converts from `lyon::extra::parser::ParseError`.
    #[error("Failed to parse SVG path data: {0}")]
    Parse(#[from] ParseError),

    /// An error indicating that fitting a curve to a set of points failed.
    #[error("Failed to fit a curve to the points")]
    FitCurve,

    /// An error indicating that cleaning a path (e.g., removing self-intersections) failed.
    #[error("Failed to clean the path")]
    CleanPath,

    /// An I/O error occurred.
    /// This is useful for operations that might read path data from files.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
