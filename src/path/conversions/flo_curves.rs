use flo_curves::{
    bezier::{
        path::{BezierPathBuilder, SimpleBezierPath},
        Curve,
    },
    BezierCurve, Coord2, Coordinate,
};
use lyon::path::Event;

use crate::path::point::PointConvert;

impl From<&crate::path::Path> for SimpleBezierPath {
    fn from(path: &crate::path::Path) -> SimpleBezierPath {
        let mut builder = BezierPathBuilder::<SimpleBezierPath>::start(Coord2::from((0.0, 0.0)));
        let mut current_pos = Coord2::from((0.0, 0.0)); // 跟踪当前位置

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
                    // 二次贝塞尔转换为三次贝塞尔控制点
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
                // --- 这是关键的修改 ---
                Event::End { first, close, .. } => {
                    // 只在 lyon 报告路径未闭合时才手动添加闭合线段
                    if !close {
                        // 同样检查一下，避免浮点误差导致添加一个极小的线段
                        if current_pos.distance_to(&first.use_as()) > 1e-6 {
                            builder = builder.line_to(first.use_as());
                        }
                    }
                    // 如果 close 为 true，我们什么都不做，因为路径已经完美闭合了。
                }
            }
        }

        builder.build()
    }
}

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

            let is_line = ctrl1 == last_point && ctrl2 == to;

            if is_line {
                builder.line_to(to.use_as());
            } else {
                builder.cubic_bezier_to(ctrl1.use_as(), ctrl2.use_as(), to.use_as());
            }

            last_point = to;
        }

        // 闭合路径：回到起点并调用 close()
        builder.line_to(start_point.use_as());
        builder.close();

        Self {
            inner: builder.build(),
        }
    }
}
