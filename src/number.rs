use core::fmt::Debug;

use num::{Float, FromPrimitive};
use simba::scalar::RealField;

pub trait Number: RealField + Float + FromPrimitive + Copy + Debug + Sync {}

impl Number for f32 {}
impl Number for f64 {}
