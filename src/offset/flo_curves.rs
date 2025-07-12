//! Implements path offsetting using the `flo_curves` library.
//!
//! This module provides the `FloCurvesOffset` struct, which uses the `flo_curves`
//! library to perform path offsetting.

use flo_curves::{
    BezierCurve, Coord2,
    bezier::{
        Curve, curve_is_tiny, fit_curve, offset,
        path::{BezierPath, BezierPathFactory, SimpleBezierPath, path_remove_interior_points},
        walk_curve_evenly,
    },
};

use crate::{
    error::{PathError, Result},
    offset::Offset,
    path::Path,
};

/// A path offsetter that uses the `flo_curves` library.
///
/// This struct encapsulates the logic for offsetting a path using the algorithms
/// provided by the `flo_curves` library.
pub struct FloCurvesOffset {
    curves: Vec<Curve<Coord2>>,
}

impl FloCurvesOffset {
    /// Creates a new `FloCurvesOffset` instance.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to the `Path` to be offset.
    /// * `offset_distance` - The distance by which to offset the path.
    pub fn new(path: &Path, offset_distance: f64) -> Self {
        FloCurvesOffset {
            curves: SimpleBezierPath::from(path)
                .to_curves()
                .into_iter()
                .flat_map(|curve| offset(&curve, -offset_distance, -offset_distance))
                .filter(|curve| !curve_is_tiny(curve))
                .collect::<Vec<_>>(),
        }
    }

    /// Returns a reference to the underlying `flo_curves` curves.
    pub fn curves(&self) -> &Vec<Curve<Coord2>> {
        &self.curves
    }
}

impl Offset for FloCurvesOffset {
    /// Offsets the path using the `flo_curves` library.
    ///
    /// This method takes the curves generated during the creation of the `FloCurvesOffset` instance,
    /// samples them, fits a new curve to the sampled points, and then cleans the resulting path
    /// to produce the final offset path.
    ///
    /// # Returns
    ///
    /// A `Result` containing the offset `Path` or an error if the offsetting process fails.
    fn offset_path(&self) -> Result<Path> {
        let offset_points = self
            .curves
            .iter()
            .flat_map(|curve| sample_curve(curve))
            .collect::<Vec<_>>();

        let fitted_curve =
            fit_curve::<Curve<Coord2>>(&offset_points, 1.0).ok_or(PathError::FitCurve)?;

        let offset_toolpath = SimpleBezierPath::from_connected_curves(
            fitted_curve
                .into_iter()
                .filter(|curve| !curve_is_tiny(curve)),
        );

        let clean_offset_toolpath: SimpleBezierPath =
            path_remove_interior_points(&vec![offset_toolpath], 0.01)
                .into_iter()
                .next()
                .ok_or(PathError::CleanPath)?;

        Ok(Path::from(&clean_offset_toolpath))
    }
}

/// Samples a Bezier curve and returns a set of representative points.
///
/// This function walks along the curve at a fixed distance and samples the midpoint
/// of each segment to generate a set of points that approximate the curve.
///
/// # Arguments
///
/// * `curve` - The Bezier curve to sample.
///
/// # Returns
///
/// A `Vec<Coord2>` containing the sampled points.
fn sample_curve(curve: &Curve<Coord2>) -> Vec<Coord2> {
    let max_error = 0.01;
    let distance = 0.1;

    // Take the midpoint (t=0.5) of each sampled section as the final sample point.
    walk_curve_evenly(curve, distance, max_error)
        .map(|section| section.point_at_pos(0.5))
        .collect::<Vec<_>>()
}
