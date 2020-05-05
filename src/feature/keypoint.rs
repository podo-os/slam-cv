use nalgebra::Point2;

use super::base::Feature;
use crate::number::Number;

pub trait KeyPoint
where
    Self: Feature,
{
    fn point_image(&self) -> Point2<Self::Number>;
}

impl<N> KeyPoint for Point2<N>
where
    N: 'static + Number,
{
    fn point_image(&self) -> Point2<Self::Number> {
        *self
    }
}

#[cfg(feature = "cv-core")]
impl KeyPoint for cv_core::KeyPoint {
    fn point_image(&self) -> Point2<Self::Number> {
        self.0
    }
}
