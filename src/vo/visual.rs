use super::ba::*;
use super::world::*;
use crate::feature::*;
use crate::number::Number;
use crate::solver::NoOptimize;

use image::GenericImageView;

#[cfg(feature = "podo")]
use podo_core_driver::Driver;

#[cfg(not(feature = "podo"))]
pub trait Driver {}

pub trait VisualSlam
where
    Self: Driver,
{
    type Number: 'static + Number;

    type KeyPoint: 'static + KeyPoint<Number = Self::Number>;
    type Descriptor: 'static + KeyPoint<Number = Self::Number> + Descriptor<Number = Self::Number>;
    type Landmark: 'static + Landmark<Number = Self::Number>;

    type Image: 'static + GenericImageView;

    type Detector: 'static
        + Detector<Number = Self::Number, KeyPoint = Self::KeyPoint, Image = Self::Image>;
    type Extractor: 'static
        + Extractor<Number = Self::Number, Feature = Self::KeyPoint, Descriptor = Self::Descriptor>;

    type World: 'static + World<Landmark = Self::Landmark>;
    type Relocalizer: 'static + Relocalizer<World = Self::World>;

    type PoseEstimator: 'static + PoseEstimator = NoOptimize;
    type LoopCloser: 'static + LoopCloser = NoOptimize;
    type BundleAdjuster: 'static + BundleAdjuster = NoOptimize;

    fn start(&self);

    fn stop(&self);

    fn pause(&self);

    fn resume(&self);

    #[cfg(feature = "serde")]
    fn load(&self);

    #[cfg(feature = "serde")]
    fn save(&self);
}
