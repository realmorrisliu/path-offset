use lyon::path::Path;

impl From<Path> for crate::path::Path {
    fn from(value: Path) -> Self {
        Self { inner: value }
    }
}
