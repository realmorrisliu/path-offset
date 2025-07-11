use flo_curves::{
    bezier::{
        curve_is_tiny, fit_curve, offset,
        path::{path_remove_interior_points, BezierPath, BezierPathFactory, SimpleBezierPath},
        walk_curve_evenly, Curve,
    },
    BezierCurve, Coord2,
};

use crate::{
    error::{PathError, Result},
    offset::Offset,
    path::Path,
};

pub struct FloCurvesOffset {
    curves: Vec<Curve<Coord2>>,
}

impl FloCurvesOffset {
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

    pub fn curves(&self) -> &Vec<Curve<Coord2>> {
        &self.curves
    }
}

impl Offset for FloCurvesOffset {
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

/// 对贝塞尔曲线进行采样，返回一组代表性点
fn sample_curve(curve: &Curve<Coord2>) -> Vec<Coord2> {
    let max_error = 0.01;
    let distance = 0.1;

    // 对每段采样结果取中点（t=0.5），作为最终采样点
    walk_curve_evenly(curve, distance, max_error)
        .map(|section| section.point_at_pos(0.5))
        .collect::<Vec<_>>()
}
