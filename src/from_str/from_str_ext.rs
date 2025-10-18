//! This module is used when FromStr is implemented in an external library which provides a function `dyn_quantity_from_str` with a C ABI

use num::Complex;
use std::str::FromStr;

use crate::{
    DynQuantity, F64RealOrComplex, ParseError, ParseErrorReason, UnitsOfSummandsNotIdentical,
};

/**
FFI representation of [`Result<DynQuantity, ParseError>`].

This crate offers the possibility of compiling the [`FromStr`] implementation
of [`DynQuantity`] into a separate static library, which is then linked to the
final binary. See the README.md for more details. The [`FromStr::from_str`]
function returns a [`Result<DynQuantity, ParseError>`] enum, which then needs
to be passed over the FFI border.

This is where this struct comes into play. It represents both variants of the
enum at once, with the discriminant being the
[`error_type`](DynQuantityOkOrErr::error_type) field. If that field has the
value 0, parsing the string was successfull and the
[`dyn_quantity`](DynQuantityOkOrErr::dyn_quantity) field contains the [`Ok`]
result (all other fields then contain meaningless default values). Otherwise,
[`dyn_quantity`](DynQuantityOkOrErr::dyn_quantity) is meaningless and the
[`ParseError`] can be reconstructed from the other fields.

An exact duplicate of this struct is part of the
"dyn_quantity_from_str_template" crate (see
`dyn_quantity/dyn_quantity_from_str_template/src/from_str/from_str_ext.rs`).
 */
#[repr(C)]
#[derive(Debug)]
struct DynQuantityOkOrErr {
    dyn_quantity: DynQuantity<Complex<f64>>,
    /**
    0 means no error. If the value is not zero, subtract 1 from this value
    to get the [`ParseError`] discriminant.
     */
    error_type: u8,
    /**
    String span where the parse error occurred (only meaningful if error_type != 0)
     */
    span: [u32; 2],
    /**
    Additional error information.
     */
    units_of_summand_not_identical: UnitsOfSummandsNotIdentical,
}

impl<V: F64RealOrComplex> FromStr for DynQuantity<V> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Call the function of the external library
        let dyn_quantity = unsafe { dyn_quantity_from_str(s.as_ptr(), s.len()) };

        let reason = match dyn_quantity.error_type {
            0 => {
                // Try to convert the Complex<f64> from DynQuantityOkOrErr to V
                match V::try_from_complexf64(dyn_quantity.dyn_quantity.value) {
                    Ok(value) => {
                        return Ok(DynQuantity::new(value, dyn_quantity.dyn_quantity.exponents));
                    }
                    Err(conversion_error) => {
                        ParseErrorReason::NotConvertibleFromComplexF64(conversion_error)
                    }
                }
            }
            1 => ParseErrorReason::UnexpectedToken,
            2 => ParseErrorReason::InputIsEmpty,
            3 => ParseErrorReason::UnbalancedBrackets,
            4 => ParseErrorReason::TwoNumbersWithoutOperator,
            5 => ParseErrorReason::TwoOperatorsWithoutNumber,
            6 => ParseErrorReason::MustNotStartWith,
            7 => ParseErrorReason::UnitsOfSummandsNotIdentical(
                dyn_quantity.units_of_summand_not_identical,
            ),
            9 => ParseErrorReason::CouldNotParse,
            _ => unreachable!(),
        };

        let span = (dyn_quantity.span[0] as usize)..(dyn_quantity.span[1] as usize);
        let substring = s[span.clone()].to_owned();
        return Err(ParseError {
            substring,
            span,
            reason,
        });
    }
}

#[link(name = "dyn_quantity_from_str")]
unsafe extern "C" {
    fn dyn_quantity_from_str(s: *const u8, len: usize) -> DynQuantityOkOrErr;
}
