use super::super::keypoint::KeyPoint;
use crate::number::Number;

use image::GenericImageView;

pub trait Detector {
    type Number: 'static + Number;
    type KeyPoint: 'static + KeyPoint<Number = Self::Number>;

    type Image: GenericImageView;
    type Args<'a>;

    fn detect<'a>(&self, img: &Self::Image, args: Self::Args<'a>) -> Vec<Self::KeyPoint>;
}
