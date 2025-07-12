//! Provides conversions to and from `flo_curves` path types.
//!
//! This module allows for interoperability with the `flo_curves` library by converting
//! between this crate's [`Path`](crate::path::Path) and `flo_curves`'s `SimpleBezierPath`
//! and `Vec<Curve<Coord2>>`. This is essential for leveraging `flo_curves`'s path
//! manipulation algorithms.

use flo_curves::{
    BezierCurve, Coord2, Coordinate,
    bezier::{
        Curve,
        path::{BezierPathBuilder, SimpleBezierPath},
    },
};
use lyon::path::Event;

use crate::path::point::PointConvert;

/// Converts a reference to a [`Path`](crate::path::Path) into a `flo_curves::SimpleBezierPath`.
///
/// This conversion processes the `lyon::path::Event` stream of the input path:
/// - `Event::Line`, `Event::Cubic`: Translated directly to `flo_curves` equivalents.
/// - `Event::Quadratic`: Mathematically converted into a cubic Bézier curve, as
///   `flo_curves` primarily works with cubic curves.
/// - `Event::End`: If the path is not marked as closed by `lyon`, a closing line segment
///   is added to ensure the `flo_curves` path is properly closed, which is often a
///   requirement for path algorithms.
impl From<&crate::path::Path> for SimpleBezierPath {
    fn from(path: &crate::path::Path) -> SimpleBezierPath {
        let mut builder = BezierPathBuilder::<SimpleBezierPath>::start(Coord2::from((0.0, 0.0)));
        let mut current_pos = Coord2::from((0.0, 0.0)); // Track current position

        for event in path.inner.iter() {
            match event {
                Event::Begin { at } => {
                    let start_point = at.use_as();
                    builder = BezierPathBuilder::start(start_point);
                    current_pos = start_point;
                }
                Event::Line { to, .. } => {
                    let to_point = to.use_as();
                    builder = builder.line_to(to_point);
                    current_pos = to_point;
                }
                Event::Quadratic { ctrl, to, .. } => {
                    // Convert quadratic Bézier to cubic control points
                    let cp1: Coord2 =
                        current_pos + (ctrl.use_as::<Coord2>() - current_pos) * (2.0 / 3.0);
                    let cp2: Coord2 = to.use_as::<Coord2>()
                        + (ctrl.use_as::<Coord2>() - to.use_as::<Coord2>()) * (2.0 / 3.0);

                    let to_point = to.use_as();
                    builder = builder.curve_to((cp1, cp2), to_point);
                    current_pos = to_point;
                }
                Event::Cubic {
                    ctrl1, ctrl2, to, ..
                } => {
                    let to_point = to.use_as();
                    builder = builder.curve_to((ctrl1.use_as(), ctrl2.use_as()), to_point);
                    current_pos = to_point;
                }
                Event::End { first, close, .. } => {
                    // Manually add a closing line segment only if lyon reports the path as open.
                    if !close {
                        // Also check to avoid adding a minuscule line due to floating point errors.
                        if current_pos.distance_to(&first.use_as()) > 1e-6 {
                            builder = builder.line_to(first.use_as());
                        }
                    }
                    // If `close` is true, do nothing, as the path is already perfectly closed.
                }
            }
        }

        builder.build()
    }
}

/// Converts a vector of `flo_curves::Curve`s into a [`Path`](crate::path::Path).
///
/// Each `Curve` is assumed to be a cubic Bézier segment. The conversion creates a
/// new `Path` where each curve becomes a separate, unclosed subpath consisting of a
/// single cubic Bézier segment.
impl From<&Vec<Curve<Coord2>>> for crate::path::Path {
    fn from(value: &Vec<Curve<Coord2>>) -> Self {
        let mut builder = lyon::path::Path::builder();

        let mut points = vec![];
        for curve in value {
            let start_point = curve.start_point();
            let end_point = curve.end_point();
            let (ctrl1, ctrl2) = curve.control_points();

            points.push((
                start_point.use_as(),
                ctrl1.use_as(),
                ctrl2.use_as(),
                end_point.use_as(),
            ));
        }

        for (start, ctrl1, ctrl2, end) in points {
            builder.begin(start);
            builder.cubic_bezier_to(ctrl1, ctrl2, end);
            builder.end(false);
        }

        Self {
            inner: builder.build(),
        }
    }
}

/// Converts a `flo_curves::SimpleBezierPath` back into a [`Path`](crate::path::Path).
///
/// This reconstructs a `lyon` path from the `flo_curves` representation. It handles
/// both lines and cubic curves. The resulting path is explicitly closed by adding a
/// line segment back to the start point and calling `close()`.
impl From<&SimpleBezierPath> for crate::path::Path {
    fn from(value: &SimpleBezierPath) -> Self {
        let (start_point, segments) = value;
        let mut builder = lyon::path::Path::builder();

        // Begin path at the start point
        builder.begin(start_point.use_as());

        // Track last point for later closure
        let mut last_point = start_point;

        for (ctrl1, ctrl2, to) in segments {
            if ctrl1.is_nan() || ctrl2.is_nan() || to.is_nan() {
                continue;
            }

            // A line is represented in SimpleBezierPath where control points align with endpoints.
            let is_line = ctrl1 == last_point && ctrl2 == to;

            if is_line {
                builder.line_to(to.use_as());
            } else {
                builder.cubic_bezier_to(ctrl1.use_as(), ctrl2.use_as(), to.use_as());
            }

            last_point = to;
        }

        // Close the path by returning to the start point.
        builder.line_to(start_point.use_as());
        builder.close();

        Self {
            inner: builder.build(),
        }
    }
}
