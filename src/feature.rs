use core::fmt::Debug;

use nalgebra::Point3;
use num::Num;

pub trait Feature {
    type Number: 'static + Num + Copy + Debug;

    fn point_world(&self) -> Point3<Self::Number>;
}
