use super::super::keypoint::KeyPoint;
use crate::number::Number;

pub trait Detector {
    type Number: 'static + Number;
    type KeyPoint: 'static + KeyPoint<Number = Self::Number>;

    type Image: 'static;

    fn detect(&self, img: &Self::Image) -> Vec<Self::KeyPoint>;
}
