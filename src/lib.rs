#![doc = include_str!("../README.md")]

pub mod cli;
pub mod diamond;
pub mod interval;
pub mod lattice;
mod math;
pub mod play;
pub mod ratio;
pub mod temperaments;

pub use lattice::{Lattice, LatticeDimension, LatticeDimensionBounds};
pub use ratio::Ratio;
pub use temperaments::Edo;
