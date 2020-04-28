use core::fmt::Debug;

use super::super::base::Feature;
use super::super::descriptor::Descriptor;

use num::Num;

pub trait Extractor {
    type Number: 'static + Num + Copy + Debug;

    type Feature: 'static + Feature<Number = Self::Number>;
    type Descriptor: 'static + Descriptor<Number = Self::Number>;
}
