dyn_quantity
============

[`DynQuantity`]: https://docs.rs/dyn_quantity/0.2.0/dyn_quantity/struct.DynQuantity.html
[`UnitExponents`]: https://docs.rs/dyn_quantity/0.2.0/dyn_quantity/struct.UnitExponents.html
[`Quantity`]: https://docs.rs/uom/latest/uom/si/struct.Quantity.html
[`from_str`]: https://docs.rs/dyn_quantity/0.2.0/dyn_quantity/from_str/index.html
[`deserialize_with`]: https://docs.rs/dyn_quantity/0.2.0/dyn_quantity/deserialize_with/index.html
[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[dyn_quantity_lexer]: https://docs.rs/dyn_quantity_lexer/latest/dyn_quantity_lexer/index.html

The strong type system of rust allows defining physical quantities as types -
see for example the [uom](https://docs.rs/uom/latest/uom/) crate. This is very
useful to evaluate the correctness of calculations at compile time. Sometimes
however, the type of a physical quantity is not known until runtime - for
example, when parsing a user-provided string (requires feature **from_str** to
be enabled). This is where this crate comes into play:

```rust
use std::str::FromStr;
use dyn_quantity::DynQuantity;

/*
Parse a string into a physical quantity. The string can contain simple 
mathematical operations as well as scientific notation. If there is no
operand specified between individual components (numbers or physical units),
multiplication is assumed. The resulting value is calculated while parsing.
*/
let quant = DynQuantity::<f64>::from_str("4e2 pi mWb / (2*s^3)^2").expect("valid");

// The SI value of the quantity is "4e2 * pi * 1e-3 / 2^2" => the 1e-3 stems 
// from the prefix "m" of "mWb". This equates to 0.1 * pi or roughly 0.31459.
assert!((quant.value - 0.31459) < 1e-5);

// The SI base units exponents of "Wb / (s^3)^2" are:
assert_eq!(quant.exponents.second, -8);
assert_eq!(quant.exponents.meter, 2);
assert_eq!(quant.exponents.kilogram, 1);
assert_eq!(quant.exponents.ampere, -1);
assert_eq!(quant.exponents.kelvin, 0);
assert_eq!(quant.exponents.mol, 0);
assert_eq!(quant.exponents.candela, 0);
```

The docstring of the [`from_str`] module provides a complete documentation of
the available parsing syntax.

# Overview

This crate is based around the [`DynQuantity`] struct, which represents a
physical quantity at runtime via its numerical value and the exponents of the
involved SI base units. The latter are fields of the struct [`UnitExponents`],
which in turn is a field of [`DynQuantity`].

The [`DynQuantity`] offers the following features:
* Performing simple arithmetic operations on quantities where the units are
only known at runtime.
* Conversion into statically-typed quantities (requires the **uom** feature to
be enabled).
* Serialization and deserialization, in case of the latter from multiple
different representations (requires the **serde** feature to be enabled).
* Parsing quantities at runtime from strings (requires the **from_str** feature
to be enabled).

# Arithmetic operations

While some operations such as multiplication, division and exponentiation are 
infallible, addition and subtraction require the unit exponents of both involved
[`DynQuantity`] structs to be identical. This is checked at runtime:

```rust
use std::str::FromStr;
use dyn_quantity::DynQuantity;

let current = DynQuantity::<f64>::from_str("-1.5 A").expect("valid");
let voltage = DynQuantity::<f64>::from_str("-3 V").expect("valid");
let power = DynQuantity::<f64>::from_str("20 W").expect("valid");

// This works: current times voltage is infallible. The resulting unit is Watt,
// therefore the subtraction succeeds
let diff = power.clone().try_sub(&(current.clone() * voltage.clone())).expect("units are compatible");
assert_eq!(diff.value, 15.5);

// This does not work: current divided by voltage squared is infallible, but the
// resulting units are not compatible to power
let res = power.try_add(&(current / voltage).powi(2));
assert!(res.is_err());
```

Another special case is root calculation: Since unit exponents can only be
integers, the exponents of the radicand ("input") need to be divisible by the
degree without remainder:

```rust
use dyn_quantity::{DynQuantity, UnitExponents};

// Create a DynQuantity from its components.
let exponents = UnitExponents {
    second: 2,
    meter: -4,
    kilogram: 0,
    ampere: 0,
    kelvin: 0,
    mol: 0,
    candela: 0,
};
let quantity = DynQuantity::new(9.0, exponents);

// Succeeds, since all exponents can be divided by 2 without remainder:
let res = quantity.clone().try_nthroot(2).expect("succeeded");
assert_eq!(res.value, 3.0);

// Fails, since not all exponents can be divided by 4 without remainder: 
assert!(quantity.try_nthroot(4).is_err());
```

# Conversion into statically-typed quantities

The uom integration is gated behind the **uom** feature flag.

One of the main features of [`DynQuantity`] is its capability to bridge the gap
between uom's [`Quantity`] type (units defined at compile time) and user-provided
input where the units are only known at runtime. For example, a user-provided
string can fallibly be parsed into a `Length`. This is a two-step operation,
where the string is first parsed into a [`DynQuantity`] and then converted
into a `Length` via `TryFrom`:

```rust
use std::str::FromStr;
use uom::si::{f64::{Length, Velocity}, length::meter};
use dyn_quantity::DynQuantity;

let input = "2 mm / s * 0.5 s";
let quantity = DynQuantity::<f64>::from_str(input).expect("valid");
let length: Length = quantity.clone().try_into().expect("valid");
assert_eq!(length.get::<meter>(), 0.001);

// Trying to convert quantity into a Velocity fails because the type does not
// match the unit exponents:
assert!(Velocity::try_from(quantity).is_err());
```

The reverse conversion from a [`Quantity`] to a [`DynQuantity`] is always
possible via the `From` implementation.

# Serialization and deserialization

The serde integration is gated behind the **serde** feature flag.

A [`DynQuantity`] can be deserialized from its "natural" struct representation
or directly from a string (by first deserializing into a string and then using
the [`FromStr`] implementation). In addition, a couple of functions for usage
with the [`deserialize_with`](https://serde.rs/field-attrs.html#deserialize_with)
field attribute are provided:

```rust
use serde::{Deserialize};
use uom::si::{f64::Length, length::meter};
use dyn_quantity::deserialize_quantity;
use indoc::indoc;

#[derive(Deserialize, Debug)]
struct LengthWrapper {
    #[serde(deserialize_with = "deserialize_quantity")]
    length: Length,
}

let ser = indoc! {"
---
length: 1200 mm
"};
let wrapper: LengthWrapper = serde_yaml::from_str(&ser).unwrap();
assert_eq!(wrapper.length.get::<meter>(), 1.2);
```
The [`deserialize_with`] module holds all available functions.

# Parsing strings

The ability to parse strings is gated behind the **from_str** feature flag.

An important part of any parser is the
[lexer](https://en.wikipedia.org/wiki/Lexical_analysis), which converts the
array of characters which make up the string into meaningful tokens. These
tokens are then later syntactically analyzed and converted to a [`DynQuantity`].
This crate uses the [logos](https://docs.rs/logos/latest/logos/) crate (inside
[dyn_quantity_lexer]) to generate a high-performance lexer via a procedural
macro at compile time. The disadvantage of this approach is the long compile
time caused by the procedural macro, hence this feature is hidden behing a
feature flag.