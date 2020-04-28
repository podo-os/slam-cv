use std::sync::Arc;

use crossbeam::atomic::AtomicCell;

pub type OptimizerController = Arc<AtomicCell<bool>>;

pub trait Optimizer {
    fn optimize(&mut self, running: OptimizerController);
}

pub struct NoOptimize;

impl Optimizer for NoOptimize {
    fn optimize(&mut self, _: OptimizerController) {}
}
