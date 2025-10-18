/*!
[`DynQuantity`]: crate::DynQuantity

# Overview

This module implements [`std::str::FromStr`] for [`DynQuantity`].
Depending on the feature "no_static_lib", this is realized in two different ways:

### Feature "no_static_lib" is enabled:

The parsing logic is compiled into a static library which is then linked into
the final binary within the [`std::str::FromStr`] implementation in the module
[`from_str_ext`].

### Feature "no_static_lib" is disabled:

The parsing logic is directly compiled into the final binary within the
[`std::str::FromStr`] implementation in the module [`from_str_impl`].

See README.md and `build.rs` for details.

# Syntax

A [`DynQuantity`] can be parsed from a string which combines numbers, units,
mathematical operators and brackets using the following syntax.

## Numbers

Numbers can be either integers or floats. Imaginary numbers need to have
either an `i` or `j` behind the numerical value (with or without a space).
For example, the following strings are all parsed to the same [`DynQuantity`]:
`2 i`, `2j`, `2.0 j`, `2.0i`.

The following special numbers are recognized:
* `inf`, `Inf`, `INF`, `infinity`, `Infinity`, `INFINITY`, `.inf`, `.Inf`,
`.INF` are all parsed to [`std::f64::INFINITY`].
* `-inf`, `-Inf`, `-INF`, `-infinity`, `-Infinity`, `-INFINITY`, `-.inf`,
`-.Inf`, `-.INF` are all parsed to [`std::f64::NEG_INFINITY`].
* `10^x` or `ex` where `x` is a positive or negative integer are parsed to `ex`
(10 to the power of `x`).
* `pi`, `π`, `PI`, `Pi` are all parsed to [`std::f64::consts::PI`].

## Units of measurement

The following units of measurement are recognized:
* `s`: Second
* `m`: Meter
* `g`: Gram
* `A`:(Ampere
* `K`: Kelvin
* `mol`: Mol
* `cd`: Candela
* `°C`: Celsius
* `V`: Volt
* `N`: Newton
* `Nm`: Newton meter
* `W`: Watt
* `J`: Joule
* `Hz`: Hertz
* `rpm`: Rotations per minute
* `Wb`: Weber
* `T`: Tesla
* `H`: Henry
* `S`: Siemens
* `t`: Ton - could also be represented by `Mg` (mega-gram)
* `Ohm`, `ohm`: Ohm
* `Ω`: Omega

Units can be prefixed by metric prefixes (see <https://en.wikipedia.org/wiki/Metric_prefix>).
This multiplies their associated numerical values with `ex`, where `x` is defined by
the following table:
* `Q`: quetta, `x` = 30
* `R`: ronna, `x` = 27
* `Y`: yotta, `x` = 24
* `Z`: zetta, `x` = 21
* `E`: exa, `x` = 18
* `P`: peta, `x` = 15
* `T`: tera, `x` = 12
* `G`: giga, `x` = 9
* `M`: mega, `x` = 6
* `k`: kilo, `x` = 3
* `d`: deci, `x` = -1
* `c`: centi, `x` = -2
* `m`: milli, `x` = -3
* `u`, `µ`: micro, `x` = -6
* `n`: nano, `x` = -9
* `p`: pico, `x` = -12
* `f`: femto, `x` = -15
* `a`: atto, `x` = -18
* `z`: zepto, `x` = -21
* `y`: yocto, `x` = -24
* `r`: ronto, `x` = -27
* `q`: quecto, `x` = -30

If a unit is raised to a power, its prefix is raised accordingly. For example,
the unit `mm^2` is equivalent to `1e-6 m^2`

## Operators

Numbers and units can be combined via mathematical operators.
While numbers always need to have an operator in between them, it is possible to
omit them when combining different units or units with numbers (a multiplication
operator is then inserted implicitly). For example, the following strings all
parse to the same [`DynQuantity`]: `3 A m`, `3 * A m`, `3 * A * m`, `3 A * m`.
Some mathematical operations are invalid when units are involved, for example
`3 A + 5 V`. Trying to parse such a string results in an
[`UnitsOfSummandsNotIdentical`](crate::UnitsOfSummandsNotIdentical) error.
The resolution of multiple operators follows the standard arithmetic rules:
exponentiation -> multiplication / division -> addition / subtraction
The following operators are available:
* `+`: Addition (fails if units of involved quantities are not identical)
* `-`: Subtraction (fails if units of involved quantities are not identical)
* `*`: Multiplication
* `/`: Division
* `^`: Exponentiation (after an exponentiation, only a positive or negative
integer may follow)
* `%`: Percentage, this is equivalent to `*1e2`

## Angles

Angles have two dimensionless units: degree or radians, which can be converted
into each other via the relationship: `angle_deg = angle_rad * 180 / pi`. When
parsing an angle, the resulting numerical value is always in radians, hence
values with the unit `degree` are converted via the aforementioned relationship.
The following strings can be used to define the angular units:
* Degree: `degree`, `Degree``, `°`, `deg`, `Deg`
* Radians: `rad`, `Rad`, `radians`, `Radians`

## Brackets

The resolution order of mathematical operations can be modified via round
brackets `(` and `)`. For each opening bracket, a corresponding closing bracket
is needed. Superfluous brackets are ignored. If no operator is specified before
a bracket, the multiplication operator `*` is inserted implicitly.

For example, the following strings parse to the same [`DynQuantity`]:
`3 * (1A + 4A)`, `3 * ((1A + 4A))`,  `3(1A + 4A)`
all result in a value of `15` with the unit `A`. Brackets are not allowed
directly after an exponentiation symbol `^`. However, exponentiation of a
bracket is allowed

# Examples

## Valid strings

```
use std::str::FromStr;
use num::Complex;
use dyn_quantity::{DynQuantity, UnitExponents};

let quantity = DynQuantity::<f64>::from_str("1 kA / m * 3.14 m^2").expect("valid string");
assert_eq!(quantity.value, 3140.0);
assert_eq!(
    quantity.exponents,
    UnitExponents {
        second: 0,
        meter: 1,
        kilogram: 0,
        ampere: 1,
        kelvin: 0,
        mol: 0,
        candela: 0
    }
);

let quantity = DynQuantity::<f64>::from_str("3e9((0.5 / kg - 1.5 / kg)) ms^3 + 2 s^3/kg").expect("valid string");
assert_eq!(quantity.value, -1.0);
assert_eq!(
    quantity.exponents,
    UnitExponents {
        second: 3,
        meter: 0,
        kilogram: -1,
        ampere: 0,
        kelvin: 0,
        mol: 0,
        candela: 0
    }
);

let quantity = DynQuantity::<Complex<f64>>::from_str("(1 A + 2i A)^2").expect("valid string");
assert_eq!(quantity.value, Complex::new(-3.0, 4.0));
assert_eq!(
    quantity.exponents,
    UnitExponents {
        second: 0,
        meter: 0,
        kilogram: 0,
        ampere: 2,
        kelvin: 0,
        mol: 0,
        candela: 0
    }
);

// It is also possible to parse a DynQuantity::<f64> from a string if all complex
// components are cancelled out
let quantity = DynQuantity::<f64>::from_str("(2i)^2").expect("valid string");
assert_eq!(quantity.value, -4.0);
assert_eq!(
    quantity.exponents,
    UnitExponents {
        second: 0,
        meter: 0,
        kilogram: 0,
        ampere: 0,
        kelvin: 0,
        mol: 0,
        candela: 0
    }
);
```

## Invalid strings

```
use std::str::FromStr;
use num::Complex;
use dyn_quantity::DynQuantity;

// Adding a dimensionless quantity to voltage fails
assert!(DynQuantity::<f64>::from_str("1 + 2V").is_err());

// Trying to parse a real DynQuantity from a string where the imaginary components
// don't cancel out fails
assert!(DynQuantity::<f64>::from_str("5i").is_err());

// Unbalanced brackets
assert!(DynQuantity::<f64>::from_str("((1 + 3)").is_err());

// Exponentiation is only allowed for integers w/o brackets
assert!(DynQuantity::<f64>::from_str("(2 km)^V").is_err());
assert!(DynQuantity::<f64>::from_str("(2 km)^(3)").is_err());

// Unknown unit
assert!(DynQuantity::<f64>::from_str("1 metre").is_err());
```
*/

#[cfg(not(feature = "no_static_lib"))]
mod from_str_ext;
#[cfg(feature = "no_static_lib")]
pub mod from_str_impl;
