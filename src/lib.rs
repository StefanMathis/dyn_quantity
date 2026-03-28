/*!
[`DynQuantity`]: crate::DynQuantity
[`Unit`]: crate::Unit
[`Quantity`]: uom::si::Quantity
[`serde_impl`]: crate::quantity::serde_impl
[`serialize_quantity`]: crate::quantity::serde_impl::serialize_quantity
[`serialize_with_units`]:crate::quantity::serde_impl::serialize_with_units
[`deserialize_quantity`]: crate::quantity::serde_impl::deserialize_quantity
[`FromStr`]: std::str::FromStr
[`from_str_impl`]: crate::quantity::from_str_impl
[dyn_quantity_lexer]: dyn_quantity_lexer

A lightweight (only one dependency with 18 SLoC) implementation of a 1d Akima
spline with optional smooth extrapolation and derivative calculation.

 */
#![doc = include_str!("../docs/main.md")]
#![deny(missing_docs)]

pub mod error;
pub mod quantity;
pub mod unit;

pub use error::*;
pub use quantity::DynQuantity;
pub use unit::{PredefUnit, Unit, UnitFromType};

#[cfg(feature = "uom")]
pub use uom;

#[cfg(feature = "serde")]
pub use quantity::serde_impl::*;
