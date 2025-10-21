/*!
This module contains the various errors which can occur when dealing with
[`DynQuantity`](crate::DynQuantity) and [`Unit`].
*/

use std::error::Error;
use std::fmt::Display;

use num::Complex;

use crate::Unit;

/**
Error representing unequality of units.

Sometimes, units of measurements must be identical for a certain operation. For
example, two physical quantities can only be added if their units are
identical. This struct holds both involved units for further inspection.
 */
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct UnitsNotEqual(pub Unit, pub Unit);

impl Display for UnitsNotEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unit {} not equal to unit {}", self.0, self.1)
    }
}

impl Error for UnitsNotEqual {}

/**
Error representing a failed attempt to calculate the `n`th root of an [`Unit`].

Calculating the `n`th root of an [`Unit`] fails if any of the exponents
is not divisible by `n`.
 */
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RootError {
    /// Root index which lead to the error.
    pub n: i32,
    /// Exponents for which the `n`th root could not be calculated.
    pub unit: Unit,
}

impl std::fmt::Display for RootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "not possible to calculate the {}th root (exponents {} cannot be divided by {} without remainder)",
            &self.n, &self.unit, &self.n
        )
    }
}

impl std::error::Error for RootError {}

/**
Error representing a failed attempt to parse a string into a
[`DynQuantity`](crate::quantity::DynQuantity).
 */
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParseError {
    /**
    String which could not be parsed
     */
    pub substring: String,
    /**
    The span can be used to index into the string to get the exact characters
    which could not be parsed.
     */
    pub span: std::ops::Range<usize>,
    /**
    Parsing can fail due to a variety of reasons, see the docstring of [`ParseErrorReason`].
     */
    pub reason: ParseErrorReason,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not parse {}: {}", &self.substring, &self.reason)
    }
}

/**
The varying reasons parsing a string to a
[`DynQuantity`](crate::quantity::DynQuantity) can fail.
This struct is part of [`ParseError`], which contains the information where
the parsing failed.
 */
#[derive(Default, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ParseErrorReason {
    /// String contained an unexpected token.
    UnexpectedToken,
    /// Input string was empty.
    InputIsEmpty,
    /// Brackets in the string are not balanced:
    /// - "((5)": Closing bracket is missing
    /// - "(5))": Opening bracket is missing
    UnbalancedBrackets,
    /// Two numbers without any combining operator are in the string:
    /// - "5 32": Invalid because it is unclear how the numbers should
    /// combined in the resulting [`DynQuantity`](crate::quantity::DynQuantity).
    /// - "5 * 32": Valid
    TwoNumbersWithoutOperator,
    /// Two operators without a number inbetween are in the string:
    /// - "3 / * 2": Invalid
    /// - "3 / 1 * 2": Valid
    TwoOperatorsWithoutNumber,
    /// The string must not start with certain characters, for example:
    /// - Operators such as *, /, ^
    /// - Closing brackets
    MustNotStartWith,
    /**
    An addition / subtraction of two invalid quantities was defined in the
    string, e.g. "3 A + 2 V".
     */
    UnitsNotEqual(UnitsNotEqual),
    /// See docstring of [`NotConvertibleFromComplexF64`].
    NotConvertibleFromComplexF64(NotConvertibleFromComplexF64),
    /// Generic fallback error for all other parsing failures
    #[default]
    CouldNotParse,
}

impl std::fmt::Display for ParseErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorReason::UnexpectedToken => {
                write!(f, "unexpected token")
            }
            ParseErrorReason::InputIsEmpty => {
                write!(f, "input is empty")
            }
            ParseErrorReason::CouldNotParse => write!(f, "could not parse the input"),
            ParseErrorReason::UnbalancedBrackets => {
                write!(f, "unbalanced number of brackets")
            }
            ParseErrorReason::TwoNumbersWithoutOperator => {
                write!(
                    f,
                    "encountered two numbers without an operator (+ or -) between them"
                )
            }
            ParseErrorReason::TwoOperatorsWithoutNumber => {
                write!(
                    f,
                    "encountered two operators (+, -, * or /) without a number between them"
                )
            }
            ParseErrorReason::UnitsNotEqual(inner) => inner.fmt(f),
            ParseErrorReason::MustNotStartWith => {
                write!(f, "input must not start with this token")
            }
            ParseErrorReason::NotConvertibleFromComplexF64(err) => err.fmt(f),
        }
    }
}

impl From<UnitsNotEqual> for ParseErrorReason {
    fn from(value: UnitsNotEqual) -> Self {
        return Self::UnitsNotEqual(value);
    }
}

impl std::error::Error for ParseErrorReason {}

/**
Error describing a failed attempt to convert a [`Complex<f64>`] into the type
`V` of [`DynQuantity<V>`](crate::quantity::DynQuantity).

For example, this error will be returned when trying to parse a string
representing a complex quantity into a
[`DynQuantity<f64>`](crate::quantity::DynQuantity).
 */
#[derive(Debug, Clone, PartialEq)]
pub struct NotConvertibleFromComplexF64 {
    /// Number which failed to convert.
    pub source: Complex<f64>,
    /// Target type name.
    pub target_type: &'static str,
}

impl std::fmt::Display for NotConvertibleFromComplexF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "could not convert from {} into target type {}",
            self.source, self.target_type
        )
    }
}

impl std::error::Error for NotConvertibleFromComplexF64 {}

/**
Error describing a failed attempt to convert between different types representing
quantities.

This error can e.g. be returned when trying to convert a
[`DynQuantity`](crate::quantity::DynQuantity) to a
[`Quantity`](https://docs.rs/uom/latest/uom/si/struct.Quantity.html) via the
[`TryFrom`] implementation. See docstring of
[`DynQuantity`](crate::quantity::DynQuantity).
*/
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    /// See docstring of [`NotConvertibleFromComplexF64`].
    NotConvertibleFromComplexF64(NotConvertibleFromComplexF64),
    /// Expected a certain unit of measurement, but found a different one.
    UnitMismatch {
        /// Unit of measurement which was expected.
        expected: Unit,
        /// Unit of measurement which was found.
        found: Unit,
    },
    /// Fallback case for all other errors.
    Custom(String),
}

impl ConversionError {
    /// Create [`Self`] from anything which can be converted to a string.
    pub fn custom<T: ToString>(err: T) -> Self {
        return Self::Custom(err.to_string());
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::NotConvertibleFromComplexF64(value) => return value.fmt(f),
            ConversionError::UnitMismatch { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
            ConversionError::Custom(string) => {
                write!(f, "{string}")
            }
        }
    }
}

impl std::error::Error for ConversionError {}
