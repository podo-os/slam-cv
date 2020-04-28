use nalgebra::Point2;

use super::base::Feature;

pub trait KeyPoint
where
    Self: Feature,
{
    fn point_image(&self) -> Point2<Self::Number>;
}
