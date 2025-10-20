// #![allow(unused)]
#![feature(trait_alias)]
#![feature(iter_map_windows)]

pub mod dielectric;
pub mod field;
pub mod point;
pub mod source;
pub mod utils;

pub use bevy_math::*;
pub use num_complex::*;
pub use rayon::iter::*;

pub use dielectric::*;
pub use field::*;
pub use point::*;
pub use source::*;
pub use utils::*;

pub const TOL: f32 = 1e-3;
