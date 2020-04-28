use nalgebra::Point3;

use super::descriptor::Descriptor;
use super::keypoint::KeyPoint;

pub trait Landmark
where
    Self: KeyPoint + Descriptor,
{
    fn point_world(&self) -> Point3<Self::Number>;
}
