use lyon::path::{Event, Iter as PathIter}; // 引入 lyon 的 Event 和迭代器类型 Iter，并重命名为 PathIter 以免混淆

/// 一个迭代器，可以将一个包含多个图形的路径分解成单个的子路径。
///
/// 这个结构体及其 Iterator 实现，封装了从一个连续的路径事件流中
/// 提取出独立子路径（从 Begin 到 End）的复杂状态管理逻辑。
pub struct SubpathIter<'a> {
    /// 持有对底层 lyon 路径事件流的迭代器。
    iter: PathIter<'a>,
}

impl<'a> Iterator for SubpathIter<'a> {
    // 每次迭代，我们都希望能产出一个完整的、我们自己的 Path 类型
    type Item = super::Path;

    /// 实现 next() 方法，这是迭代器的核心。
    /// 每次调用，它会尝试构建并返回下一个完整的子路径。
    fn next(&mut self) -> Option<Self::Item> {
        // 这个 next() 方法的逻辑，是您提供的原始 for 循环的直接翻译。

        // 1. 创建一个 Path Builder 来构建子路径。
        //    我们从流中寻找下一个 `Begin` 事件来启动它。
        let mut builder;
        if let Some(event) = self.iter.find(|e| matches!(e, Event::Begin { .. })) {
            if let Event::Begin { at } = event {
                // 找到了起点，初始化 builder
                let mut b = lyon::path::Path::builder();
                b.begin(at);
                builder = b;
            } else {
                // 理论上不可能发生，因为 find 已经保证了是 Begin
                return None;
            }
        } else {
            // 如果在整个事件流中再也找不到 Begin 事件，说明迭代结束了。
            return None;
        }

        // 2. 既然已经有了启动的 builder，我们继续消耗事件流，
        //    直到遇到对应的 `End` 事件。
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
                    // 遇到了 End，一个子路径构建完成。
                    if close {
                        builder.close();
                    }
                    // 构建出 lyon::path::Path，包装成我们自己的 Path，然后返回。
                    // next() 方法的本次执行到此结束。
                    return Some(super::Path {
                        inner: builder.build(),
                    });
                }
                Event::Begin { .. } => {
                    // 如果在找到 End 之前又遇到了 Begin，
                    // 这意味着上一个子路径没有正常结束。
                    // 根据原始逻辑，我们应该结束当前构建并开始新的。
                    // 在迭代器模式下，最简单的处理方式是就此打住，
                    // 让下一次 next() 调用来处理这个新的 Begin 事件。
                    // 但这意味着我们可能会丢失一个未闭合的路径。
                    //
                    // 为了忠实于您的原始代码（它会隐式地丢弃未完成的路径），
                    // 我们在这里直接 break，然后返回 None。
                    break;
                }
            }
        }

        // 如果 for 循环正常结束（意味着迭代器耗尽了），
        // 但我们还没返回，说明最后一个子路径没有 End 事件。
        // 您的原始代码会忽略这种情况，所以我们也返回 None。
        None
    }
}

/// 为我们自己的 Path 类型的引用实现 IntoIterator trait。
/// 这使得可以直接在 for 循环中使用 `&Path`。
impl<'a> IntoIterator for &'a super::Path {
    type Item = super::Path;
    type IntoIter = SubpathIter<'a>;

    /// 定义如何从一个 `&Path` 创建出我们的 `SubpathIter` 迭代器。
    fn into_iter(self) -> Self::IntoIter {
        SubpathIter {
            iter: self.inner.iter(),
        }
    }
}
