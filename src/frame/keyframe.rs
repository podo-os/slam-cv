use crate::feature::{Feature, Landmark};
use crate::number::Number;

use nalgebra::Isometry3;

pub trait KeyFrame {
    type Number: 'static + Number;
    type Feature: 'static + Feature<Number = Self::Number>;

    fn for_landmarks<F>(&self, f: F)
    where
        Self::Feature: 'static + Landmark,
        F: FnMut(&Self::Feature);

    fn isometry(&self) -> Isometry3<Self::Number>;
}
