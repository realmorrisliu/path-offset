pub mod cavalier_contours;
pub mod flo_curves;

use crate::{error::Result, path::Path};

pub trait Offset {
    fn offset_path(&self) -> Result<Path>;
}
