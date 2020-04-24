use crate::feature::Feature;

// TODO: detach Visual-SLAM (features, descriptors) from SLAM
pub trait World {
    type Feature: 'static + Feature;

    fn for_features<F>(&self, f: F)
    where
        F: FnMut(&Self::Feature);
}
