#![doc = include_str!("../README.md")]

pub mod from_str;

#[cfg(feature = "uom")]
pub mod uom_impl;

#[cfg(feature = "serde")]
pub mod deserialize_with;

include!("../dyn_quantity_from_str/common/lib.rs");
