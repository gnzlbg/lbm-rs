#![feature(conservative_impl_trait)]
#![feature(slice_patterns)]

#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

extern crate time;
pub extern crate rayon;

// TODO: parametrize on floating point type (currently num)

#[allow(non_camel_case_types)]
type num = f64;


mod traits;
use traits::*;
pub use traits::Distribution;
pub use traits::DistributionStorage;


pub mod physics;
pub mod distribution;
mod solver;
pub use solver::Solver;
pub mod geometry;
pub mod grid;
pub mod boundary;
pub mod io;
