#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

pub trait PointConvert {
    // 定义一个泛型转换方法
    fn use_as<T>(&self) -> T
    where
        // T 必须能从我们的标准 Point 转换而来
        T: From<Point>,
        // Self 必须能被转换成我们的标准 Point
        Point: From<Self>,
        Self: Copy; // 假设所有点类型都是 Copy
}

impl<P> PointConvert for P
where
    P: Copy,
    Point: From<P>,
{
    fn use_as<T>(&self) -> T
    where
        T: From<Point>,
    {
        // 核心逻辑：通过我们的标准 Point 作为中介
        let canonical_point = Point::from(*self);
        T::from(canonical_point)
    }
}

impl From<lyon::math::Point> for Point {
    fn from(value: lyon::math::Point) -> Self {
        Self(value.x as f64, value.y as f64)
    }
}
impl From<Point> for lyon::math::Point {
    fn from(point: Point) -> Self {
        lyon::geom::euclid::point2(point.0 as f32, point.1 as f32)
    }
}

impl From<flo_curves::bezier::Coord2> for Point {
    fn from(value: flo_curves::bezier::Coord2) -> Self {
        Self(value.0, value.1)
    }
}
impl From<Point> for flo_curves::bezier::Coord2 {
    fn from(point: Point) -> Self {
        flo_curves::bezier::Coord2(point.0, point.1)
    }
}
