#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod error;
pub mod quantity;
pub mod unit;

pub use error::*;
pub use quantity::DynQuantity;
pub use unit::{CommonUnits, Unit, UnitFromType};

#[cfg(feature = "serde")]
pub use quantity::serde_impl::*;
