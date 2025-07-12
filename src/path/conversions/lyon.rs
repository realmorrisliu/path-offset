//! Provides conversions from `lyon::path::Path`.
//!
//! This module facilitates the direct use of `lyon` paths within this crate
//! by providing a `From` implementation.

use lyon::path::Path;

/// Converts a `lyon::path::Path` into this crate's [`Path`](crate::path::Path) type.
///
/// Since `crate::path::Path` is a wrapper around `lyon::path::Path`, this is a
/// straightforward conversion.
impl From<Path> for crate::path::Path {
    fn from(value: Path) -> Self {
        Self { inner: value }
    }
}
