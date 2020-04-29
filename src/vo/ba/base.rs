use crate::solver::{NoOptimize, Optimizer};

pub trait BundleAdjuster: Optimizer {}

impl BundleAdjuster for NoOptimize {}
