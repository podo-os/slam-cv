use core::fmt::Debug;

use super::super::keypoint::KeyPoint;

use num::Num;

pub trait Detector {
    type Number: 'static + Num + Copy + Debug;
    type KeyPoint: 'static + KeyPoint;
}
