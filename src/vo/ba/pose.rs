use crate::solver::{NoOptimize, Optimizer};

pub trait PoseEstimator: Optimizer {}

impl PoseEstimator for NoOptimize {}
