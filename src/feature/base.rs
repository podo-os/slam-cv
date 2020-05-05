use crate::number::Number;

use nalgebra::{Point2, Point3};

pub trait Feature {
    type Number: 'static + Number;
}

impl<N> Feature for Point2<N>
where
    N: 'static + Number,
{
    type Number = N;
}

impl<N> Feature for Point3<N>
where
    N: 'static + Number,
{
    type Number = N;
}

#[cfg(feature = "cv-core")]
impl Feature for cv_core::KeyPoint {
    type Number = f64;
}
