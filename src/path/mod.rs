//! Defines the `Path` struct and related utilities for path manipulation.
//!
//! This module provides the core `Path` struct, which represents a geometric path,
//! and includes functionality for parsing, manipulating, and iterating over paths.

use std::{fmt::Display, str::FromStr};

use lyon::path::Event;

use crate::error::PathError;

pub mod conversions;
pub mod point;
pub mod subpath;

/// Represents a geometric path, composed of one or more subpaths.
///
/// A `Path` can be created from an SVG path string and can be iterated over
/// to process its individual subpaths. It also provides utilities for
/// analyzing path properties, such as finding the outermost contour.
#[derive(Debug, Clone)]
pub struct Path {
    inner: lyon::path::Path,
}

impl Path {
    /// Returns an iterator over the subpaths of this path.
    ///
    /// Each item in the iterator is a `Path` representing a single subpath.
    pub fn iter(&self) -> impl Iterator<Item = Path> + '_ {
        self.into_iter()
    }

    /// Checks if the path is closed.
    ///
    /// A path is considered closed if it ends with a `Close` event.
    pub fn is_closed(&self) -> bool {
        self.inner
            .iter()
            .any(|e| matches!(e, Event::End { close: true, .. }))
    }

    /// Find and return the subpath that represents the outermost shell.
    ///
    /// This method first attempts to use a fast "largest area" heuristic.
    /// If that fails to produce a result, it falls back to a more accurate but slower
    /// "geometric containment" algorithm.
    ///
    /// # Returns
    ///
    /// An `Option<Path>` containing the outermost shell if found, otherwise `None`.
    pub fn find_outer_shell(&self) -> Option<Path> {
        let subpaths: Vec<Path> = self.iter().collect();

        match subpaths.len() {
            // Case 1: No subpaths
            0 => None,

            // Case 2: Only one subpath, which is the shell by definition.
            // We use .into_iter().next() to consume the Vec and take the single element
            // without needing to clone it.
            1 => subpaths.into_iter().next(),

            // Case 3: Multiple subpaths, execute the "smart" finding logic.
            _ => {
                // First, try the fast area heuristic.
                find_shell_by_area(&subpaths)
                    // If the area method returns nothing, fall back to the precise geometric containment algorithm.
                    .or_else(|| find_shell_by_containment(&subpaths))
            }
        }
    }

    /// Checks if this path's bounding box intersects with another path's bounding box.
    fn intersect_with(&self, other: &Path) -> bool {
        let bbox_a = lyon::algorithms::aabb::bounding_box(self.inner.iter());
        let bbox_b = lyon::algorithms::aabb::bounding_box(other.inner.iter());
        bbox_a.intersects(&bbox_b)
    }

    /// Checks if this path is geometrically contained within another path.
    fn contained_by(&self, other_path: &Path) -> bool {
        // A path cannot contain itself.
        !std::ptr::eq(self, other_path)
            // Both paths must be closed to have a well-defined interior.
            && self.is_closed()
            && other_path.is_closed()
            // Check if the first point of this path is inside the other path.
            && self.inner.first_endpoint().map_or(false, |(pt, _)| {
                lyon::algorithms::hit_test::hit_test_path(
                    &pt,
                    &other_path.inner,
                    lyon::path::FillRule::EvenOdd,
                    0.1,
                )
            })
    }
}

/// Parses a `Path` from an SVG path data string.
///
/// # Errors
///
/// Returns a `PathError` if the SVG path data is invalid.
impl FromStr for Path {
    type Err = PathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = lyon::extra::parser::PathParser::new();
        let mut builder = lyon::path::Path::builder();
        let mut src = lyon::extra::parser::Source::new(s.chars());

        parser.parse(
            &lyon::extra::parser::ParserOptions::DEFAULT,
            &mut src,
            &mut builder,
        )?;

        let path = builder.build();
        Ok(Path::from(path))
    }
}

/// Formats the `Path` as an SVG path data string.
impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_slice = self.inner.as_slice();

        for event in path_slice.iter_with_attributes() {
            match event {
                Event::Begin { at: (at, _) } => {
                    write!(f, "M{},{}", at.x, at.y)?;
                }
                Event::Line { to: (to, _), .. } => {
                    write!(f, "L{},{}", to.x, to.y)?;
                }
                Event::Quadratic {
                    ctrl, to: (to, _), ..
                } => {
                    write!(f, "Q{},{} {},{}", ctrl.x, ctrl.y, to.x, to.y)?;
                }
                Event::Cubic {
                    ctrl1,
                    ctrl2,
                    to: (to, _),
                    ..
                } => {
                    write!(
                        f,
                        "C{},{} {},{} {},{}",
                        ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y
                    )?;
                }
                Event::End { close, .. } => {
                    if close {
                        write!(f, "Z")?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// Strategy 1: Find the outermost shell by calculating signed area.
/// This is a fast heuristic.
fn find_shell_by_area(paths: &[Path]) -> Option<Path> {
    paths
        .iter()
        // Only consider closed paths, as only they can define an inside and outside.
        .filter(|p| p.is_closed())
        .max_by(|a, b| {
            let area_a = lyon::algorithms::area::approximate_signed_area(0.01, a.inner.iter());
            let area_b = lyon::algorithms::area::approximate_signed_area(0.01, b.inner.iter());
            // total_cmp can handle special f32 cases like NaN and infinity.
            area_a.total_cmp(&area_b)
        })
        .cloned()
}

/// Strategy 2: Find the outermost shell by checking for geometric containment.
/// This is a precise but computationally more expensive algorithm.
fn find_shell_by_containment(paths: &[Path]) -> Option<Path> {
    paths
        .iter()
        .find(|this_path| {
            // Find a path that is not contained by any other path.
            !paths.iter().any(|other_path| {
                // Use our previously defined helper methods.
                this_path.intersect_with(other_path) && this_path.contained_by(other_path)
            })
        })
        .cloned()
}
