//! Defines the canonical `Point` type and a generic conversion trait `PointConvert`.
//!
//! This module provides a standardized `Point` struct that serves as an intermediary
//! for converting between different point representations from various geometry libraries.
//! The `PointConvert` trait enables seamless, generic conversion of any point type
//! that can be converted to and from the canonical `Point`.

/// A canonical 2D point representation with `f64` precision.
///
/// This struct acts as a common ground for converting between point types
/// from different libraries (e.g., `lyon::math::Point`, `flo_curves::bezier::Coord2`).
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

/// A trait for generically converting between different point types.
///
/// Any type that implements `Copy` and has `From` implementations to and from
/// the canonical [`Point`] struct will automatically implement this trait.
/// It provides a `use_as` method to convert an instance of a point type into another
/// point type, using [`Point`] as the intermediary.
pub trait PointConvert {
    /// Converts the point into a different point type `T`.
    ///
    /// This method leverages the canonical [`Point`] struct as a bridge. The conversion
    /// follows the path: `Self` -> `Point` -> `T`.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The target point type. It must be convertible from [`Point`].
    ///
    /// # Constraints
    ///
    /// * `T` must implement `From<Point>`.
    /// * The canonical [`Point`] must implement `From<Self>`.
    /// * `Self` must be `Copy`.
    fn use_as<T>(&self) -> T
    where
        T: From<Point>,
        Point: From<Self>,
        Self: Copy;
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
        // The core logic: convert `self` to the canonical `Point`, then to the target type `T`.
        let canonical_point = Point::from(*self);
        T::from(canonical_point)
    }
}

/// Converts a `lyon::math::Point` to the canonical `Point`.
impl From<lyon::math::Point> for Point {
    fn from(value: lyon::math::Point) -> Self {
        Self(value.x as f64, value.y as f64)
    }
}

/// Converts the canonical `Point` to a `lyon::math::Point`.
impl From<Point> for lyon::math::Point {
    fn from(point: Point) -> Self {
        lyon::geom::euclid::point2(point.0 as f32, point.1 as f32)
    }
}

/// Converts a `flo_curves::bezier::Coord2` to the canonical `Point`.
impl From<flo_curves::bezier::Coord2> for Point {
    fn from(value: flo_curves::bezier::Coord2) -> Self {
        Self(value.0, value.1)
    }
}

/// Converts the canonical `Point` to a `flo_curves::bezier::Coord2`.
impl From<Point> for flo_curves::bezier::Coord2 {
    fn from(point: Point) -> Self {
        flo_curves::bezier::Coord2(point.0, point.1)
    }
}
