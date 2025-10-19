/*!
This module offers a [`TryFrom`] implementation to convert from a [`DynQuantity`]
to the statically-typed [`Quantity`] struct from the [`uom`] crate. This is a
fallible operation, since the unit of measurement of [`DynQuantity`] is only
known at runtime, while the unit of measurement of [`Quantity`] is defined
at compile time via the type system.

In constrast, conversion from a [`Quantity`] to a [`DynQuantity`] is infallible
and is therefore realized via a [`From`] implementation.
*/

/**
A trait to derive [`UnitExponents`] from a type. This trait bridges the gap
between (external) types representing physical quantities (such as e.g. the
[`Quantity`](uom::si::Quantity) type from the [`uom`] crate) and [`UnitExponents`].
 */
pub trait AsUnitExponents {
    /**
    This function derives an [`UnitExponents`] from any type which implements
    [`AsUnitExponents`]. Its default implementation returns an [`UnitExponents`]
    where all exponents are zero.

    # Examples
    ```
    use dyn_quantity::AsUnitExponents;
    use uom::si::f64::Length;

    // 64-bit floats do not represent a physical quantity
    let exp = f64::as_unit_exponents();
    assert_eq!(exp.meter, 0);

    // The "Length" type alias from the uom crate represents a physical quantity (length)
    let exp = Length::as_unit_exponents();
    assert_eq!(exp.meter, 1);
    ```
    */
    fn as_unit_exponents() -> UnitExponents {
        return UnitExponents::default();
    }
}

impl AsUnitExponents for f64 {}

impl AsUnitExponents for Complex<f64> {}

#[cfg(feature = "uom")]
impl<L, M, T, I, Th, N, J, K> AsUnitExponents
    for uom::si::Quantity<uom::si::ISQ<L, M, T, I, Th, N, J, K>, uom::si::SI<f64>, f64>
where
    L: uom::typenum::Integer,
    M: uom::typenum::Integer,
    T: uom::typenum::Integer,
    I: uom::typenum::Integer,
    Th: uom::typenum::Integer,
    N: uom::typenum::Integer,
    J: uom::typenum::Integer,
    K: ?Sized,
{
    fn as_unit_exponents() -> UnitExponents {
        return UnitExponents {
            second: T::to_i32(),
            meter: L::to_i32(),
            kilogram: M::to_i32(),
            ampere: I::to_i32(),
            kelvin: Th::to_i32(),
            mol: N::to_i32(),
            candela: J::to_i32(),
        };
    }
}

use num::{Zero, complex::Complex};
use uom::si::*;

use crate::{
    ConversionError, DynQuantity, F64RealOrComplex, NotConvertibleFromComplexF64, UnitExponents,
};

impl<L, M, T, I, Th, N, J, K, V> TryFrom<DynQuantity<V>>
    for uom::si::Quantity<uom::si::ISQ<L, M, T, I, Th, N, J, K>, uom::si::SI<f64>, f64>
where
    L: uom::typenum::Integer,
    M: uom::typenum::Integer,
    T: uom::typenum::Integer,
    I: uom::typenum::Integer,
    Th: uom::typenum::Integer,
    N: uom::typenum::Integer,
    J: uom::typenum::Integer,
    K: ?Sized,
    V: F64RealOrComplex,
{
    type Error = ConversionError;

    fn try_from(quantity: DynQuantity<V>) -> Result<Self, Self::Error> {
        // Check dimensional correctness (compare runtime to compile-time unit exponents)
        let expected = UnitExponents {
            second: T::to_i32(),
            meter: L::to_i32(),
            kilogram: M::to_i32(),
            ampere: I::to_i32(),
            kelvin: Th::to_i32(),
            mol: N::to_i32(),
            candela: J::to_i32(),
        };
        if expected != quantity.exponents {
            return Err(ConversionError::UnitMismatch {
                expected,
                found: quantity.exponents,
            });
        }

        // Construct the uom quantity directly from raw data. This is feasible since si_value() converts the quantity value to a coherent SI value
        // (e.g by converting 1 km into 1000 m)
        let value = quantity.value.to_complexf64();

        // Return an error if the value contains a complex component
        if !value.im.is_zero() {
            return Err(ConversionError::NotConvertibleFromComplexF64(
                NotConvertibleFromComplexF64 {
                    source: value,
                    target_type: "f64",
                },
            ));
        }

        return Ok(uom::si::Quantity {
            dimension: std::marker::PhantomData,
            units: std::marker::PhantomData,
            value: value.re,
        });
    }
}

impl<L, M, T, I, Th, N, J, K, V> TryFrom<DynQuantity<V>>
    for uom::si::Quantity<
        uom::si::ISQ<L, M, T, I, Th, N, J, K>,
        uom::si::SI<Complex<f64>>,
        Complex<f64>,
    >
where
    L: uom::typenum::Integer,
    M: uom::typenum::Integer,
    T: uom::typenum::Integer,
    I: uom::typenum::Integer,
    Th: uom::typenum::Integer,
    N: uom::typenum::Integer,
    J: uom::typenum::Integer,
    K: ?Sized,
    V: F64RealOrComplex,
{
    type Error = ConversionError;

    fn try_from(quantity: DynQuantity<V>) -> Result<Self, Self::Error> {
        // Check dimensional correctness (compare runtime to compile-time unit exponents)
        let expected = UnitExponents {
            second: T::to_i32(),
            meter: L::to_i32(),
            kilogram: M::to_i32(),
            ampere: I::to_i32(),
            kelvin: Th::to_i32(),
            mol: N::to_i32(),
            candela: J::to_i32(),
        };
        if expected != quantity.exponents {
            return Err(ConversionError::UnitMismatch {
                expected,
                found: quantity.exponents,
            });
        }

        // Construct the uom quantity directly from raw data. This is feasible
        // since value converts the quantity value to a coherent SI value
        // (e.g by converting 1 km into 1000 m)
        let value = quantity.value.to_complexf64();

        return Ok(uom::si::Quantity {
            dimension: std::marker::PhantomData,
            units: std::marker::PhantomData,
            value,
        });
    }
}

impl<L, M, T, I, Th, N, J, K, V>
    From<uom::si::Quantity<uom::si::ISQ<L, M, T, I, Th, N, J, K>, uom::si::SI<f64>, f64>>
    for DynQuantity<V>
where
    L: uom::typenum::Integer,
    M: uom::typenum::Integer,
    T: uom::typenum::Integer,
    I: uom::typenum::Integer,
    Th: uom::typenum::Integer,
    N: uom::typenum::Integer,
    J: uom::typenum::Integer,
    K: ?Sized,
    V: F64RealOrComplex,
{
    fn from(
        quantitiy: uom::si::Quantity<uom::si::ISQ<L, M, T, I, Th, N, J, K>, uom::si::SI<f64>, f64>,
    ) -> Self {
        let exponents = UnitExponents {
            second: T::to_i32(),
            meter: L::to_i32(),
            kilogram: M::to_i32(),
            ampere: I::to_i32(),
            kelvin: Th::to_i32(),
            mol: N::to_i32(),
            candela: J::to_i32(),
        };
        return DynQuantity::new(V::from_f64(quantitiy.value), exponents);
    }
}

impl<L, M, T, I, Th, N, J, K, V>
    TryFrom<
        uom::si::Quantity<
            uom::si::ISQ<L, M, T, I, Th, N, J, K>,
            uom::si::SI<Complex<f64>>,
            Complex<f64>,
        >,
    > for DynQuantity<V>
where
    L: uom::typenum::Integer,
    M: uom::typenum::Integer,
    T: uom::typenum::Integer,
    I: uom::typenum::Integer,
    Th: uom::typenum::Integer,
    N: uom::typenum::Integer,
    J: uom::typenum::Integer,
    K: ?Sized,
    V: F64RealOrComplex,
{
    type Error = NotConvertibleFromComplexF64;

    fn try_from(
        quantitiy: uom::si::Quantity<
            uom::si::ISQ<L, M, T, I, Th, N, J, K>,
            uom::si::SI<Complex<f64>>,
            Complex<f64>,
        >,
    ) -> Result<Self, Self::Error> {
        let value = V::try_from_complexf64(quantitiy.value)?;

        let exponents = UnitExponents {
            second: T::to_i32(),
            meter: L::to_i32(),
            kilogram: M::to_i32(),
            ampere: I::to_i32(),
            kelvin: Th::to_i32(),
            mol: N::to_i32(),
            candela: J::to_i32(),
        };
        return Ok(DynQuantity::new(value, exponents));
    }
}
