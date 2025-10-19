// #![allow(unused)]
#![feature(trait_alias)]
#![feature(iter_map_windows)]

mod dielectric;
mod field;
mod point;
mod source;
mod utils;

pub use bevy_math::*;
pub use dielectric::*;
pub use field::*;
pub use num_complex::*;
pub use point::*;
pub use source::*;
pub use utils::*;

pub const TOL: f32 = 1e-3;
