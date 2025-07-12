//! Defines the `Offset` trait for path offsetting.
//!
//! This module provides the central `Offset` trait, which defines the contract for path offsetting algorithms.
//! It also includes modules for different offsetting implementations, such as `cavalier_contours` and `flo_curves`.

pub mod cavalier_contours;
pub mod flo_curves;

use crate::{error::Result, path::Path};

/// A trait for types that can offset a path.
///
/// This trait provides a generic interface for path offsetting algorithms.
/// Implementors of this trait are expected to provide an implementation for the `offset_path` method.
pub trait Offset {
    /// Offsets the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to the `Path` to be offset.
    ///
    /// # Returns
    ///
    /// A `Result` containing the offset `Path` or an error.
    fn offset_path(&self) -> Result<Path>;
}
