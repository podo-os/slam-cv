#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]

pub mod color;
pub mod feature;
pub mod frame;
pub mod matcher;
mod number;
pub mod solver;
pub mod vo;

pub use number::Number;

pub mod prelude {
    pub use super::color::Colors;
    pub use super::feature::*;
    pub use super::frame::*;
    pub use super::matcher::*;
    pub use super::solver::*;
    pub use super::vo::*;
    pub use super::Number;
}
