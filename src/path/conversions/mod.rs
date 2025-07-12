//! Handles conversions between this crate's path types and those from other geometry libraries.
//!
//! This module and its submodules provide `From` trait implementations to facilitate
//! interoperability. For example, converting a `lyon::path::Path` into this crate's
//! `path_offset::path::Path` and vice-versa.
//!
//! Currently supported libraries:
//! - [`lyon`](lyon)
//! - [`flo_curves`](flo_curves)
//! - [`cavalier_contours`](cavalier_contours) (placeholder)

pub mod cavalier_contours;
pub mod flo_curves;
pub mod lyon;
