#![feature(associated_type_defaults)]

pub mod color;
pub mod feature;
pub mod frame;
pub mod matcher;
pub mod slam;
pub mod solver;

pub mod prelude {
    pub use super::color::Colors;
    pub use super::feature::*;
    pub use super::frame::*;
    pub use super::matcher::*;
    pub use super::slam::VisualSlam;
    pub use super::solver::*;
}
