//! # path-offset
//! A simple library for offsetting paths.
//!
//! This library provides tools to perform path offsetting, a common operation in computer graphics and CAD.
//! It leverages other powerful libraries for robust geometric calculations and provides a simple and flexible API.
//!
//! ## Features
//!
//! - **Path Offsetting**: Easily offset complex paths using different strategies.
//! - **Multiple Backends**: Choose between `flo_curves` and `cavalier_contours` for the offsetting algorithm.
//! - **Path Utilities**: Includes utilities for path manipulation, such as finding the outer shell of a complex path.
//! - **SVG Path Support**: Parse SVG path data and convert paths back to SVG path strings.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! path-offset = "0.1.1"
//! ```
//!
//! ### Offsetting a Path
//!
//! ```rust
//! use path_offset::offset::Offset;
//! use path_offset::path::Path;
//! use std::str::FromStr;
//!
//! let path = Path::from_str("M10,10 L20,10 L20,20 L10,20 Z").unwrap();
//!
//! // Use one of the available offsetters
//! let offsetter = path_offset::offset::cavalier_contours::CavalierContours::new(1.0);
//! let offset_path = offsetter.offset_path(&path).unwrap();
//!
//! println!("Offset path: {}", offset_path);
//! ```

pub mod error;
pub mod offset;
pub mod path;
