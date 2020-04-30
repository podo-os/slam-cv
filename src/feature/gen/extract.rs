use super::super::base::Feature;
use super::super::descriptor::Descriptor;
use crate::number::Number;

pub trait Extractor {
    type Number: 'static + Number;

    type Feature: 'static + Feature<Number = Self::Number>;
    type Descriptor: 'static + Descriptor<Number = Self::Number>;
}
