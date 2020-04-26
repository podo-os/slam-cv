use nalgebra::Point3;

use super::base::Feature;

pub trait Landmark
where
    Self: Feature,
{
    fn point_world(&self) -> Point3<Self::Number>;
}
