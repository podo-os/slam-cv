use crate::solver::{NoOptimize, Optimizer};

pub trait LoopCloser: Optimizer {}

impl LoopCloser for NoOptimize {}
