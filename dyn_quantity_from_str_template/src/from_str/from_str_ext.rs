use crate::{DynQuantity, UnitsOfSummandsNotIdentical};
use num::Complex;
use std::{slice, str::FromStr};

/**
FFI representation of [`Result<DynQuantity, ParseError>`]. Must be identical
to the struct of the same name in dyn_quantity/src/from_str/from_str_ext.rs.
See the docstring of that struct.
 */
#[repr(C)]
#[derive(Debug)]
struct DynQuantityOkOrErr {
    dyn_quantity: DynQuantity<Complex<f64>>,
    error_type: u8,
    span: [u32; 2],
    units_of_summand_not_identical: UnitsOfSummandsNotIdentical,
}

/**
SAFETY: The caller needs to make sure that:
1) ptr is a pointer to a string
2) len is the length of the string (without /0 terminator if it is a C string)
If this function is called from rust, those properties can be looked up with the `as_ptr` and `len` methods of the &str type.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dyn_quantity_from_str(ptr: *const u8, len: usize) -> DynQuantityOkOrErr {
    // Rebuild &str from a pointer and the slice length
    let result = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(ptr, len);

        // ... and then convert that slice into a string slice
        std::str::from_utf8(slice)
    };

    let s = match result {
        Ok(s) => s,
        Err(_) => {
            return DynQuantityOkOrErr {
                dyn_quantity: Default::default(),
                error_type: 8,
                span: Default::default(),
                units_of_summand_not_identical: Default::default(),
            };
        }
    };

    match DynQuantity::from_str(s) {
        Ok(dyn_quantity) => {
            return DynQuantityOkOrErr {
                dyn_quantity,
                error_type: 0,
                span: [0, 0],
                units_of_summand_not_identical: Default::default(),
            };
        }
        Err(err) => {
            let error_type = err.reason.discriminant() + 1;
            let span = [err.span.start as u32, err.span.end as u32];
            let dyn_quantity = DynQuantity::default();
            let units_of_summand_not_identical = match err.reason {
                crate::ParseErrorReason::UnitsOfSummandsNotIdentical(
                    units_of_summands_not_identical,
                ) => units_of_summands_not_identical,
                _ => Default::default(),
            };
            return DynQuantityOkOrErr {
                dyn_quantity,
                error_type,
                span,
                units_of_summand_not_identical,
            };
        }
    }
}
