use std::{fmt::Display, str::FromStr};

use lyon::path::Event;

use crate::error::PathError;

pub mod conversions;
pub mod point;
pub mod subpath;

#[derive(Debug, Clone)]
pub struct Path {
    inner: lyon::path::Path,
}

impl Path {
    pub fn iter(&self) -> impl Iterator<Item = Path> + '_ {
        self.into_iter()
    }

    pub fn is_closed(&self) -> bool {
        self.inner
            .iter()
            .any(|e| matches!(e, Event::End { close: true, .. }))
    }

    /// 智能地查找并返回代表最外层轮廓的子路径。
    ///
    /// 这个方法会优先使用快速的“面积最大”启发式算法。
    /// 如果该算法无法找到结果，则会回退到更精确但更慢的“几何包含”算法。
    pub fn find_outer_shell(&self) -> Option<Path> {
        let subpaths: Vec<Path> = self.iter().collect();

        match subpaths.len() {
            // 情况一：没有子路径
            0 => None,

            // 情况二：只有一个子路径，那它自身就是外壳
            // 我们用 .into_iter().next() 来消耗 Vec 并取出唯一的元素
            // 这样可以避免克隆（.clone()）。
            1 => subpaths.into_iter().next(),

            // 情况三：有多个子路径，执行我们的“智能”查找逻辑
            _ => {
                // 首先尝试快速的面积启发式算法
                find_shell_by_area(&subpaths)
                    // 如果面积法没有返回任何结果，则回退到精确的几何包含算法
                    .or_else(|| find_shell_by_containment(&subpaths))
            }
        }
    }

    fn intersect_with(&self, other: &Path) -> bool {
        let bbox_a = lyon::algorithms::aabb::bounding_box(self.inner.iter());
        let bbox_b = lyon::algorithms::aabb::bounding_box(other.inner.iter());
        bbox_a.intersects(&bbox_b)
    }

    fn contained_by(&self, other_path: &Path) -> bool {
        !std::ptr::eq(self, other_path)
            && self.is_closed()
            && other_path.is_closed()
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

/// 策略一：通过计算有向面积找到最外层轮廓。
/// 这是一个快速的启发式算法。
fn find_shell_by_area(paths: &[Path]) -> Option<Path> {
    paths
        .iter()
        // 只考虑闭合路径，因为只有闭合路径能定义内外
        .filter(|p| p.is_closed())
        .max_by(|a, b| {
            let area_a = lyon::algorithms::area::approximate_signed_area(0.01, a.inner.iter());
            let area_b = lyon::algorithms::area::approximate_signed_area(0.01, b.inner.iter());
            // total_cmp 可以处理 f32 的 NaN 和无穷大等特殊情况
            area_a.total_cmp(&area_b)
        })
        .cloned() // 从 &Path 得到 Path
}

/// 策略二：通过检查几何包含关系找到最外层轮廓。
/// 这是一个精确但计算成本较高的算法。
fn find_shell_by_containment(paths: &[Path]) -> Option<Path> {
    paths
        .iter()
        .find(|this_path| {
            // 寻找一个不被任何其他路径包含的路径
            !paths.iter().any(|other_path| {
                // 使用我们之前设计好的辅助方法
                this_path.intersect_with(other_path) && this_path.contained_by(other_path)
            })
        })
        .cloned()
}
