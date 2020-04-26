use core::fmt::Debug;

use num::Num;

pub trait Feature {
    type Number: 'static + Num + Copy + Debug;
}
