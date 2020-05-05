#![feature(associated_type_defaults)]

mod color;
pub mod feature;
pub mod frame;
pub mod matcher;
mod number;
pub mod solver;
pub mod vo;

pub use self::color::Colors;
pub use self::number::Number;

pub mod prelude {
    pub use super::feature::*;
    pub use super::frame::*;
    pub use super::matcher::*;
    pub use super::solver::*;
    pub use super::vo::*;
    pub use super::{Colors, Number};
}
