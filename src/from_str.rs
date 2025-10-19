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

use ::num::{Complex, Zero};
use dyn_quantity_lexer::{Logos, Token};
use std::{
    f64::{INFINITY, NEG_INFINITY, consts::PI},
    str::FromStr,
};

use crate::*;

impl<V: F64RealOrComplex> FromStr for DynQuantity<V> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dyn_quantity = from_str_complexf64(s)?;
        match V::try_from_complexf64(dyn_quantity.value) {
            Ok(value) => {
                return Ok(DynQuantity::new(value, dyn_quantity.exponents));
            }
            Err(conversion_error) => {
                return Err(ParseError {
                    substring: "".into(),
                    span: 0..0,
                    reason: ParseErrorReason::NotConvertibleFromComplexF64(conversion_error),
                });
            }
        }
    }
}

fn from_str_complexf64(s: &str) -> Result<DynQuantity<Complex<f64>>, ParseError> {
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum PreviousToken {
        Add,
        Sub,
        Mul,
        Div,
        Other,
    }

    /**
    This enum encapsulates a quantity and the mathematical operation which follows it.
    For example:
    Add(x) means "x +", Mul(x) means "x *", Div(x) means "x /".
     */
    #[derive(Debug)]
    enum Operation {
        Add(DynQuantity<Complex<f64>>),
        Mul(DynQuantity<Complex<f64>>),
        Div(DynQuantity<Complex<f64>>),
    }

    impl From<Operation> for DynQuantity<Complex<f64>> {
        fn from(value: Operation) -> Self {
            match value {
                Operation::Add(item) => item,
                Operation::Mul(item) => item,
                Operation::Div(item) => item,
            }
        }
    }

    fn adjust<F: FnMut(&mut DynQuantity<Complex<f64>>)>(
        active_quantity: &mut Option<DynQuantity<Complex<f64>>>,
        mut fun: F,
    ) {
        let mut quantity = active_quantity
            .take()
            .unwrap_or(DynQuantity::new(Complex::new(1.0, 0.0), Default::default()));

        fun(&mut quantity);

        *active_quantity = Some(quantity);
    }

    fn include_infinity(active_quantity: &mut Option<DynQuantity<Complex<f64>>>, infinity: f64) {
        if let Some(quantity) = active_quantity.as_mut() {
            let re = if quantity.value.re == 0.0 {
                0.0
            } else {
                quantity.value.re.signum() * infinity
            };
            let im = if quantity.value.im == 0.0 {
                0.0
            } else {
                quantity.value.re.signum() * infinity
            };
            quantity.value = Complex::new(re, im);
        } else {
            *active_quantity = Some(DynQuantity::new(
                Complex::new(infinity, 0.0),
                Default::default(),
            ));
        }
    }

    /**
    When multiplying an infinite value with zero, the IEEE result is NaN.
    However, in our case the result should be 0
     */
    fn multiply_no_nan(arg1: Complex<f64>, arg2: Complex<f64>) -> Complex<f64> {
        // Multiply "by hand" in order to treat 0 * Inf accordingly
        let mut re = 0.0;
        let mut im = 0.0;

        if !(arg1.re.is_infinite() && arg2.re.is_zero())
            && !(arg1.re.is_zero() && arg2.re.is_infinite())
        {
            re += arg1.re * arg2.re;
        }

        if !(arg1.im.is_infinite() && arg2.re.is_zero())
            && !(arg1.im.is_zero() && arg2.re.is_infinite())
        {
            im += arg1.im * arg2.re;
        }

        if !(arg1.re.is_infinite() && arg2.im.is_zero())
            && !(arg1.re.is_zero() && arg2.im.is_infinite())
        {
            im += arg1.re * arg2.im;
        }

        if !(arg1.im.is_infinite() && arg2.im.is_zero())
            && !(arg1.im.is_zero() && arg2.im.is_infinite())
        {
            re -= arg1.im * arg2.im;
        }

        return Complex::new(re, im);
    }

    // ===============================================================================

    let mut lexer = Token::lexer(s);

    let mut active_quantity: Option<DynQuantity<Complex<f64>>> = None;

    // This is a stack of quantities. Two quantities are separated by a mathematical operator
    // which defines how the quantities are combined. THe last operator is combined with "active_quantity".
    // For example, stack = [Add(x), Mul(y)] and active_quantity = Some(z), where x, y and z are quantities would
    // be combined as follows:
    // x + y * z
    let mut stack: Vec<Operation> = Vec::new();

    // The bracket level increases by one for each open bracket "(" and decreases by one for each closing
    // bracket ")". At the end of the parsing, the bracket level must be zero. Additionally, it may never become
    // negative, therefore we always use checked_sub. If it would become negative or is not zero at the end,
    // an error is returned.
    let mut bracket_level: usize = 0;

    let mut previous_token = PreviousToken::Other;
    let mut division_pending = false;

    while let Some(token) = lexer.next() {
        let token: Token = token.map_err(|_| {
            let reason = ParseErrorReason::UnexpectedToken;
            return ParseError {
                substring: s[lexer.span()].to_owned(),
                span: lexer.span(),
                reason,
            };
        })?;

        match token {
            Token::Real(val) => {
                if let Some(quantity) = active_quantity.as_mut() {
                    quantity.value = multiply_no_nan(quantity.value, Complex::new(val, 0.0));
                } else {
                    active_quantity =
                        Some(DynQuantity::new(Complex::new(val, 0.0), Default::default()));
                }
            }
            Token::Imag(val) => {
                if let Some(quantity) = active_quantity.as_mut() {
                    quantity.value = multiply_no_nan(quantity.value, Complex::new(0.0, val));
                } else {
                    active_quantity =
                        Some(DynQuantity::new(Complex::new(0.0, val), Default::default()));
                }
            }
            Token::Infinity => {
                include_infinity(&mut active_quantity, INFINITY);
            }
            Token::NegInfinity => {
                include_infinity(&mut active_quantity, NEG_INFINITY);
            }
            Token::Mul => {
                // This is essentially a no-op - we therefore do just some error checking
                if previous_token != PreviousToken::Other {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                if active_quantity.is_none() {
                    let reason = ParseErrorReason::MustNotStartWith;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                previous_token = PreviousToken::Mul;
                continue;
            }
            Token::Div => {
                if previous_token != PreviousToken::Other {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                if let Some(quantity) = active_quantity.take() {
                    stack.push(Operation::Div(quantity));
                } else {
                    let reason = ParseErrorReason::MustNotStartWith;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                previous_token = PreviousToken::Div;
                division_pending = true;

                // Short-circuit to retain previous_token and division_pending into the next division
                continue;
            }
            Token::Percent => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *= 1e-2;
                });
            }
            Token::Pi(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *= PI.powi(exponents.unit) * 10.0.powi(exponents.exponent());
                });
            }
            Token::LeftBracket => {
                if let Some(quantity) = active_quantity.take() {
                    stack.push(Operation::Mul(quantity));
                } else {
                    /*
                    If a bracket starts and the previous token was not an operator, add
                    a buffer operation (multiplication with one) to the stack so the stack popping
                    stops at the opening bracket when the closing (right) bracket comes.
                     */
                    if previous_token == PreviousToken::Other {
                        stack.push(Operation::Mul(DynQuantity::new(
                            Complex::new(1.0, 0.0),
                            Default::default(),
                        )));
                    }
                }
                bracket_level += 1;
            }
            Token::RightBracket(exponent) => match bracket_level.checked_sub(1) {
                // Merge all stack elements with the current bracket up to and including the first "multiply".
                Some(val) => {
                    if let Some(mut quantity) = active_quantity.take() {
                        while let Some(stack_item) = stack.pop() {
                            match stack_item {
                                Operation::Add(elem) => {
                                    quantity.try_add_assign(&elem).map_err(|add| {
                                        return ParseError {
                                            substring: s[lexer.span()].to_owned(),
                                            span: lexer.span(),
                                            reason: ParseErrorReason::UnitsOfSummandsNotIdentical(
                                                add,
                                            ),
                                        };
                                    })?
                                }
                                Operation::Mul(elem) => {
                                    quantity = elem * quantity.powi(exponent);
                                    break;
                                }
                                Operation::Div(elem) => {
                                    quantity = elem / quantity.powi(exponent);
                                    break;
                                }
                            }
                        }

                        // The resolved bracket becomes the new active quantity
                        active_quantity = Some(quantity);

                        // Adjust the bracket level
                        bracket_level = val;
                    } else {
                        let reason = ParseErrorReason::UnbalancedBrackets;
                        return Err(ParseError {
                            substring: s[lexer.span()].to_owned(),
                            span: lexer.span(),
                            reason,
                        });
                    }
                }
                None => {
                    let reason = ParseErrorReason::UnbalancedBrackets;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
            },
            Token::Add => {
                // Constructs such as "-+" or "++" are not allowed, but *+1 is
                if previous_token == PreviousToken::Add || previous_token == PreviousToken::Sub {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }

                let new_quantity = DynQuantity::new(Complex::new(1.0, 0.0), Default::default());
                if let Some(quantity) = active_quantity.replace(new_quantity) {
                    stack.push(Operation::Add(quantity));
                }
                previous_token = PreviousToken::Add;
                continue;
            }
            Token::Sub => {
                // Constructs such as "+-" or "--" are not allowed, but /-1 or /+1 is
                if previous_token == PreviousToken::Add || previous_token == PreviousToken::Sub {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }

                let new_quantity = DynQuantity::new(Complex::new(-1.0, 0.0), Default::default());
                if let Some(quantity) = active_quantity.replace(new_quantity) {
                    stack.push(Operation::Add(quantity));
                }
                previous_token = PreviousToken::Sub;
                continue;
            }
            Token::PowerOfTen(exponent) => {
                if let Some(quantity) = active_quantity.as_mut() {
                    quantity.value *= 10.0.powi(exponent);
                } else {
                    active_quantity = Some(DynQuantity::new(
                        Complex::new(10.0.powi(exponent), 0.0),
                        Default::default(),
                    ));
                }
            }
            Token::Second(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.second += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Meter(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.meter += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Gram(mut exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    // Special treatment of gram: The prefix needs to be reduced by 3, since the SI system works in kilogram
                    exponents.prefix -= 3;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Ampere(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.ampere += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Kelvin(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kelvin += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Mol(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.mol += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Candela(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.candela += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Celsius(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kelvin += exponents.unit;
                    // Special treatment of celsius: The value needs to be corrected by an offset of -273.15 to the power of the unit exponent
                    quantity.value += (273.15f64).powi(exponents.unit);
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Newton(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Watt(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 3 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Joule(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Volt(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.ampere -= exponents.unit;
                    quantity.exponents.second -= 3 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Weber(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.ampere -= exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Tesla(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.ampere -= exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Henry(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.ampere -= 2 * exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Hertz(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.second -= exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Siemens(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram -= exponents.unit;
                    quantity.exponents.meter -= 2 * exponents.unit;
                    quantity.exponents.second += 3 * exponents.unit;
                    quantity.exponents.ampere += 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::RotationsPerMinute(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.second -= exponents.unit;
                    quantity.value *=
                        (1.0f64 / 60.0f64).powi(exponents.unit) * 10.0.powi(exponents.exponent());
                });
            }
            Token::Degree(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *=
                        (PI / 180.0).powi(exponents.unit) * 10.0.powi(exponents.exponent());
                });
            }
            Token::Radians(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Ohm(exponents) | Token::Omega(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 3 * exponents.unit;
                    quantity.exponents.ampere -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::NewtonMeter(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Ton(mut exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    // Special treatment of gram: The prefix needs to be increased by 3
                    exponents.prefix += 3;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
        }

        // If the last element of the stack is a division and the next token was no open bracket,
        // perform the division immediately
        if division_pending {
            if let Some(last_stack_item) = stack.last() {
                if let Operation::Div(_) = last_stack_item {
                    if let Some(quantity) = active_quantity.take() {
                        // Remove the last element of the stack, so it can be used in the division
                        let popped_quantity: DynQuantity<Complex<f64>> =
                            stack.pop().expect("stack has at least one element").into();

                        // Perform the division
                        active_quantity = Some(popped_quantity / quantity);
                    }
                }
            } else {
                // Division without stack item would mean that the string looks something like this: "/3"
                // This results in a parse error
                let reason = ParseErrorReason::UnbalancedBrackets;
                return Err(ParseError {
                    substring: s[lexer.span()].to_owned(),
                    span: lexer.span(),
                    reason,
                });
            }
        }
        division_pending = false;
        previous_token = PreviousToken::Other;
    }
    // End of the loop

    // Check the bracket level
    if bracket_level != 0 {
        let reason = ParseErrorReason::UnbalancedBrackets;
        return Err(ParseError {
            substring: s[lexer.span()].to_owned(),
            span: lexer.span(),
            reason,
        });
    }

    if let Some(initial) = stack.pop() {
        let initial = if let Some(quantity) = active_quantity.take() {
            stack.push(initial);
            quantity
        } else {
            initial.into()
        };

        stack.into_iter().try_fold(initial, |mut acc, item| {
            match item {
                Operation::Add(item) => acc.try_add_assign(&item).map_err(|add| {
                    return ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason: ParseErrorReason::UnitsOfSummandsNotIdentical(add),
                    };
                })?,
                Operation::Mul(item) => {
                    acc = item * acc;
                }
                Operation::Div(item) => {
                    acc = item / acc;
                }
            }
            Ok(acc)
        })
    } else {
        if let Some(quantity) = active_quantity.take() {
            return Ok(quantity);
        } else {
            let reason = ParseErrorReason::InputIsEmpty;
            return Err(ParseError {
                substring: s[lexer.span()].to_owned(),
                span: lexer.span(),
                reason,
            });
        }
    }
}
