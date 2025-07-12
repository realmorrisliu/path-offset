//! Provides an iterator to decompose a `Path` into its individual subpaths.
//!
//! A `Path` can contain multiple disconnected shapes (e.g., the letter 'i' has two).
//! This module provides the [`SubpathIter`] iterator, which is created via the
//! [`IntoIterator`] implementation for `&Path`. This allows you to easily loop
//! over each continuous segment of a larger path.
//!
//! # Example
//!
//! ```no_run
//! use path_offset::path::Path;
//! use lyon::path::Path as LyonPath;
//!
//! // Create a path with two separate subpaths.
//! let mut builder = LyonPath::builder();
//! builder.begin(lyon::math::point(0.0, 0.0));
//! builder.line_to(lyon::math::point(10.0, 0.0));
//! builder.end(false); // First subpath
//! builder.begin(lyon::math::point(20.0, 0.0));
//! builder.line_to(lyon::math::point(30.0, 0.0));
//! builder.end(false); // Second subpath
//! let lyon_path = builder.build();
//!
//! let path = Path::from(lyon_path);
//!
//! // Iterate over the subpaths.
//! let mut subpath_count = 0;
//! for subpath in &path {
//!     subpath_count += 1;
//!     // Each `subpath` is a `path_offset::path::Path` containing one continuous shape.
//! }
//!
//! assert_eq!(subpath_count, 2);
//! ```

use lyon::path::{Event, Iter as PathIter};

/// An iterator that decomposes a path containing multiple shapes into individual subpaths.
///
/// This struct and its `Iterator` implementation encapsulate the state management
/// required to extract independent subpaths (from a `Begin` to an `End` event)
/// from a continuous stream of path events.
///
/// It is typically not used directly, but rather through the `for` loop syntax on a `&Path`.
pub struct SubpathIter<'a> {
    /// Holds an iterator over the underlying `lyon` path's event stream.
    iter: PathIter<'a>,
}

impl<'a> Iterator for SubpathIter<'a> {
    // Each iteration yields a complete `Path` object representing one subpath.
    type Item = super::Path;

    /// Implements the core logic of the iterator.
    ///
    /// Each call attempts to build and return the next complete subpath from the
    /// underlying event stream.
    fn next(&mut self) -> Option<Self::Item> {
        // 1. Find the next `Begin` event to start a new subpath builder.
        let mut builder;
        if let Some(event) = self.iter.find(|e| matches!(e, Event::Begin { .. })) {
            if let Event::Begin { at } = event {
                // Found a start point, initialize the builder.
                let mut b = lyon::path::Path::builder();
                b.begin(at);
                builder = b;
            } else {
                // This is theoretically unreachable because `find` ensures it's a Begin event.
                return None;
            }
        } else {
            // No more `Begin` events are found in the stream, so iteration is complete.
            return None;
        }

        // 2. With an active builder, consume events until the corresponding `End` event is found.
        for event in &mut self.iter {
            match event {
                Event::Line { to, .. } => {
                    builder.line_to(to);
                }
                Event::Quadratic { ctrl, to, .. } => {
                    builder.quadratic_bezier_to(ctrl, to);
                }
                Event::Cubic {
                    ctrl1, ctrl2, to, ..
                } => {
                    builder.cubic_bezier_to(ctrl1, ctrl2, to);
                }
                Event::End { close, .. } => {
                    // An `End` event signifies a complete subpath.
                    if close {
                        builder.close();
                    }
                    // Build the lyon::path::Path, wrap it in our own Path type, and return it.
                    // This concludes the current call to next().
                    return Some(super::Path {
                        inner: builder.build(),
                    });
                }
                Event::Begin { .. } => {
                    // If another `Begin` is encountered before an `End`, the previous
                    // subpath was not properly terminated. In an iterator context,
                    // the simplest approach is to stop here and let the next call to `next()`
                    // process this new `Begin` event. This means the unclosed path is discarded.
                    break;
                }
            }
        }

        // If the loop finishes without returning, it means the iterator was exhausted
        // but the last subpath did not have a corresponding `End` event.
        // This incomplete subpath is ignored, and we return None.
        None
    }
}

/// Implements the `IntoIterator` trait for references to our `Path` type.
///
/// This is what allows a `&Path` to be used directly in a `for` loop,
/// transparently creating a [`SubpathIter`] to drive the iteration.
impl<'a> IntoIterator for &'a super::Path {
    type Item = super::Path;
    type IntoIter = SubpathIter<'a>;

    /// Defines how to create a [`SubpathIter`] from a `&Path`.
    fn into_iter(self) -> Self::IntoIter {
        SubpathIter {
            iter: self.inner.iter(),
        }
    }
}
