use core::fmt::Debug;

use super::super::keypoint::KeyPoint;

use image::GenericImageView;
use num::Num;

pub trait Detector {
    type Number: 'static + Num + Copy + Debug;
    type KeyPoint: 'static + KeyPoint<Number = Self::Number>;

    type Image: GenericImageView;
    type Args<'a>;

    fn detect<'a>(&self, img: &Self::Image, args: Self::Args<'a>) -> Vec<Self::KeyPoint>;
}
