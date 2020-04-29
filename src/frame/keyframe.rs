use crate::feature::{Feature, Landmark};

pub trait KeyFrame {
    type Feature: 'static + Feature;

    fn for_landmarks<F>(&self, f: F)
    where
        Self::Feature: 'static + Landmark,
        F: FnMut(&Self::Feature);
}
