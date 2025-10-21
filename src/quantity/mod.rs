/*!
This module contains the [`DynQuantity`] struct and supporting code. See the
documentation string of [`DynQuantity`] for more information.
*/

use num::Complex;
use num::complex::ComplexFloat;

use std::ops::{Div, DivAssign, Mul, MulAssign};

use crate::error::{ConversionError, NotConvertibleFromComplexF64, RootError, UnitsNotEqual};
use crate::unit::Unit;

#[cfg(feature = "from_str")]
pub mod from_str_impl;

#[cfg(feature = "serde")]
pub mod serde_impl;

#[cfg(feature = "uom")]
pub mod uom_impl;

mod private {
    use super::Complex;

    pub trait Sealed {}

    impl Sealed for f64 {}
    impl Sealed for Complex<f64> {}
}

/**
This is an internal trait which is used to convert between [`f64`] and
[`Complex<f64>`]-based [`DynQuantity`] structs. It needs to be public because
it is part of the type signature of [`DynQuantity`], but it is not meant to be
implemented by external types and is therefore sealed.
*/
pub trait F64RealOrComplex:
    ComplexFloat<Real: std::fmt::Display>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::fmt::Display
    + std::ops::Mul<f64, Output = Self>
    + std::ops::MulAssign
    + std::ops::MulAssign<f64>
    + std::ops::Div<f64, Output = Self>
    + std::ops::DivAssign
    + std::ops::DivAssign<f64>
    + std::fmt::Debug
    + private::Sealed
{
    /**
    Tries to convert from a [`Complex<f64>`] to the implementor of this trait,
    either [`Complex<f64>`] or [`f64`]. The former conversion always succeeds
    (is a no-op), while the latter fails if `number` has an imaginary component.
     */
    fn try_from_complexf64(number: Complex<f64>) -> Result<Self, NotConvertibleFromComplexF64>;

    /**
    Converts a [`Complex<f64>`] or [`f64`] to a [`Complex<f64>`]. This is
    infallible (and a no-op in case of the former conversion).
     */
    fn to_complexf64(self) -> Complex<f64>;

    /**
    Converts a [`f64`] to a [`Complex<f64>`] or [`f64`]. This is infallible
    (and a no-op in case of the latter conversion).
     */
    fn from_f64(value: f64) -> Self;

    /**
    Sets the real part of the implementor to `value`.
     */
    fn set_re_f64(&mut self, value: f64) -> ();

    /**
    Sets the imaginary part of the implementor to `value`. If the implementing
    type is [`f64`], this is a no-op.
     */
    fn set_im_f64(&mut self, value: f64) -> ();

    /**
    Calcute the `n`th root of `self`.
     */
    fn nth_root(self, n: i32) -> Self;
}

impl F64RealOrComplex for f64 {
    fn try_from_complexf64(number: Complex<f64>) -> Result<Self, NotConvertibleFromComplexF64> {
        if number.im() == 0.0 {
            return Ok(number.re());
        } else {
            return Err(NotConvertibleFromComplexF64 {
                source: number,
                target_type: "f64",
            });
        }
    }

    fn to_complexf64(self) -> Complex<f64> {
        return Complex::new(self, 0.0);
    }

    fn from_f64(value: f64) -> Self {
        return value;
    }

    fn set_re_f64(&mut self, value: f64) -> () {
        *self = value;
    }

    fn set_im_f64(&mut self, _: f64) -> () {
        ();
    }

    fn nth_root(self, n: i32) -> Self {
        return self.powf(1.0 / n as f64);
    }
}

impl F64RealOrComplex for Complex<f64> {
    fn try_from_complexf64(number: Complex<f64>) -> Result<Self, NotConvertibleFromComplexF64> {
        return Ok(Complex::<f64>::from(number));
    }

    fn to_complexf64(self) -> Complex<f64> {
        return self;
    }

    fn from_f64(value: f64) -> Self {
        return Complex::new(value, 0.0);
    }

    fn set_re_f64(&mut self, value: f64) -> () {
        self.re = value;
    }

    fn set_im_f64(&mut self, value: f64) -> () {
        self.im = value;
    }

    fn nth_root(self, n: i32) -> Self {
        return self.powf(1.0 / n as f64);
    }
}

/**
This type represents a physical quantity via its numerical `value` and a unit of
measurement (field `exponents`). The unit of measurement is not defined via the
type system, but rather via the values of the [`Unit`]. This means that
the unit of measurement is not fixed at compile time, but can change dynamically
at runtime.

This property is very useful when e.g. parsing a user-provided string to a
physical quantity. For this case, [`DynQuantity`] implements the
[`FromStr`](`std::str::FromStr`) trait. The module documentation
[`from_str`](crate::quantity::from_str_impl) has more information regarding the
available syntax.

The `V` generic parameter needs to implement [`F64RealOrComplex`].
Currently, implementors are [`f64`] and [`Complex<f64>`]. It is possible to
convert between those two types using the methods provided by [`F64RealOrComplex`].
In general, you likely want `V` to be [`f64`] except when dealing with complex
quantities such as alternating currents.

This struct can also be pretty-print the quantity it represents via its
[`std::fmt::Display`] implementation:

```
use std::str::FromStr;
use dyn_quantity::DynQuantity;

let quantity = DynQuantity::<f64>::from_str("9.81 m/s^2").expect("parseable");
assert_eq!(quantity.to_string(), "9.81 s^-2 m".to_string());
```

# Conversion into uom `Quantity`

If the `uom` feature is enabled, a [`DynQuantity`] can be (fallible)
converted into a
[`Quantity`](https://docs.rs/uom/latest/uom/si/struct.Quantity.html) via
[`TryFrom`]. In combination with the aforementioned parsing capabilities, this
allows fallible parsing of strings to statically-typed physical quantities.

# Serialization and deserialization

If the `serde` feature is enabled, this struct can be serialized and
deserialized. Serialization creates the "standard"
[serde](https://crates.io/crates/serde) representation one would expect from the
`Serialize` macro.

When deserializing however, multiple options are available:
1) Using the "standard" serialized representation of a struct. For example, the
yaml representation of a [`DynQuantity<f64>`] looks like this:
```text
---
value: 2.0
exponents:
    second: 0
    meter: 1
    kilogram: 0
    ampere: 1
    kelvin: 0
    mol: 0
    candela: 0
```

2) Deserializing directly from a string. This uses the [`std::str::FromStr`]
implementation under the hood, see the
[`from_str`](crate::quantity::from_str_impl) module documentation. Only
available if the  `from_str ` feature is enabled.
3) Deserialize directly from a real or complex value. This option is mainly here
to allow deserializing a serialized [uom](https://crates.io/crates/uom) quantity
(whose serialized representation is simply its numerical value without any
units). For example, deserializing `5.0` into [`DynQuantity<f64>`] produces the
same result as deserializing:
```text
---
value: 5.0
exponents:
    second: 0
    meter: 0
    kilogram: 0
    ampere: 0
    kelvin: 0
    mol: 0
    candela: 0
```

The three different possibilities are realized via a crate-internal untagged enum.
*/
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct DynQuantity<V: F64RealOrComplex> {
    /**
    The value of the physical quantity.
     */
    pub value: V,
    /**
    The (SI) base units of the physical quantity, represented by their exponents.
     */
    pub unit: Unit,
}

impl<V: F64RealOrComplex> DynQuantity<V> {
    /**
    Returns a new instance of `Self`.
     */
    pub fn new(value: V, unit: Unit) -> Self {
        return Self { value, unit };
    }

    /**
    Fallible addition of `self` and `other`.

    Physical quantities can only be added together if their units are identical.
    Hence, this function first compares the `.unit` fields of `self` and
    `other`. If they are identical, the `value` fields are added up and the
    resulting quantity is returned. Otherwise, a [`UnitsNotEqual`]
    error is returned.

    # Examples
    ```
    use std::str::FromStr;
    use dyn_quantity::DynQuantity;

    let curr1 = DynQuantity::<f64>::from_str("1 A").expect("valid");
    let curr2 = DynQuantity::<f64>::from_str("1 V*A / V").expect("valid");
    let volt1 = DynQuantity::<f64>::from_str("-5 V").expect("valid");

    // The currents can be added ...
    let curr_sum = curr1.try_add(&curr2).expect("can be added");
    assert_eq!(curr_sum.value, 2.0);
    assert_eq!(curr_sum.unit.ampere, 1);

    // ... but adding a current to a voltage fails.
    assert!(volt1.try_add(&curr1).is_err());
    ```
     */
    pub fn try_add(&self, other: &Self) -> Result<Self, UnitsNotEqual> {
        let mut output = self.clone();
        output.try_add_assign(other)?;
        return Ok(output);
    }

    /**
    Like [`DynQuantity::try_add`], but assigns the sum of `self` and `other` to
    `self` instead of returning it. If the addition fails, `self` is not modified.

    # Examples
    ```
    use std::str::FromStr;
    use dyn_quantity::DynQuantity;

    let mut curr1 = DynQuantity::<f64>::from_str("1 A").expect("valid");
    let curr2 = DynQuantity::<f64>::from_str("1 V*A / V").expect("valid");
    let mut volt1 = DynQuantity::<f64>::from_str("-5 V").expect("valid");

    // curr1 gets overwritten
    curr1.try_add_assign(&curr2).expect("can be added");
    assert_eq!(curr1.value, 2.0);
    assert_eq!(curr1.unit.ampere, 1);

    // volt1 does not get modified because the addition failed
    let volt_cpy = volt1.clone();
    assert!(volt1.try_add_assign(&curr1).is_err());
    assert_eq!(volt1, volt_cpy);
    ```
     */
    pub fn try_add_assign(&mut self, other: &Self) -> Result<(), UnitsNotEqual> {
        if self.unit == other.unit {
            self.value += other.value;
            return Ok(());
        } else {
            return Err(UnitsNotEqual(self.unit.clone(), other.unit.clone()));
        }
    }

    /**
    Fallible subtraction of `self` and `other`.

    Physical quantities can only be subtracted from each other if their units are identical.
    Hence, this function first compares the `.unit` fields of `self` and
    `other`. If they are identical, the `value` fields are subtracted up and the
    resulting quantity is returned. Otherwise, a [`UnitsNotEqual`]
    error is returned.

    # Examples
    ```
    use std::str::FromStr;
    use dyn_quantity::DynQuantity;

    let curr1 = DynQuantity::<f64>::from_str("1 A").expect("valid");
    let curr2 = DynQuantity::<f64>::from_str("1 V*A / V").expect("valid");
    let volt1 = DynQuantity::<f64>::from_str("-5 V").expect("valid");

    // The currents can be subtracted ...
    let curr_sum = curr1.try_sub(&curr2).expect("can be added");
    assert_eq!(curr_sum.value, 0.0);
    assert_eq!(curr_sum.unit.ampere, 1);

    // ... but sbutracting a current from a voltage fails.
    assert!(volt1.try_sub(&curr1).is_err());
    ```
     */
    pub fn try_sub(&self, other: &Self) -> Result<Self, UnitsNotEqual> {
        let mut output = self.clone();
        output.try_sub_assign(other)?;
        return Ok(output);
    }

    /**
    Like [`DynQuantity::try_sub`], but assigns the difference of `self` and `other` to
    `self` instead of returning it. If the subtraction fails, `self` is not modified.

    # Examples
    ```
    use std::str::FromStr;
    use dyn_quantity::DynQuantity;

    let mut curr1 = DynQuantity::<f64>::from_str("1 A").expect("valid");
    let curr2 = DynQuantity::<f64>::from_str("1 V*A / V").expect("valid");
    let mut volt1 = DynQuantity::<f64>::from_str("-5 V").expect("valid");

    // curr1 gets overwritten
    curr1.try_sub_assign(&curr2).expect("can be added");
    assert_eq!(curr1.value, 0.0);
    assert_eq!(curr1.unit.ampere, 1);

    // volt1 does not get modified because the addition failed
    let volt_cpy = volt1.clone();
    assert!(volt1.try_sub_assign(&curr1).is_err());
    assert_eq!(volt1, volt_cpy);
    ```
     */
    pub fn try_sub_assign(&mut self, other: &Self) -> Result<(), UnitsNotEqual> {
        if self.unit == other.unit {
            self.value -= other.value;
            return Ok(());
        } else {
            return Err(UnitsNotEqual(self.unit.clone(), other.unit.clone()));
        }
    }

    /**
    Raises `self` to an integer power.

    # Examples
    ```
    use std::str::FromStr;
    use dyn_quantity::DynQuantity;

    let curr = DynQuantity::<f64>::from_str("2 A^2").expect("valid");
    let result = curr.powi(3);
    assert_eq!(result.value, 8.0);
    assert_eq!(result.unit.ampere, 6);
    ```
     */
    pub fn powi(mut self, n: i32) -> Self {
        self.value = self.value.powi(n);
        self.unit = self.unit.powi(n);
        return self;
    }

    /**
    Tries to calculate the `n`th root of self.
     */
    pub fn try_nthroot(mut self, n: i32) -> Result<Self, RootError> {
        self.unit = self.unit.try_nthroot(n)?;
        self.value = self.value.nth_root(n);
        return Ok(self);
    }
}

impl<V: F64RealOrComplex> std::fmt::Display for DynQuantity<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.im() == V::zero().im() {
            write!(f, "{}", self.value.re())?;
        } else {
            write!(f, "({})", self.value)?;
        }

        // Go through all units and add them, if their exponents aren't zero
        if self.unit.second != 0 {
            if self.unit.second == 1 {
                write!(f, " s")?;
            } else {
                write!(f, " s^{}", self.unit.second)?;
            }
        }
        if self.unit.meter != 0 {
            if self.unit.meter == 1 {
                write!(f, " m")?;
            } else {
                write!(f, " m^{}", self.unit.meter)?;
            }
        }
        if self.unit.kilogram != 0 {
            if self.unit.kilogram == 1 {
                write!(f, " kg")?;
            } else {
                write!(f, " kg^{}", self.unit.kilogram)?;
            }
        }
        if self.unit.ampere != 0 {
            if self.unit.ampere == 1 {
                write!(f, " A")?;
            } else {
                write!(f, " A^{}", self.unit.ampere)?;
            }
        }
        if self.unit.kelvin != 0 {
            if self.unit.kelvin == 1 {
                write!(f, " K")?;
            } else {
                write!(f, " K^{}", self.unit.kelvin)?;
            }
        }
        if self.unit.mol != 0 {
            if self.unit.mol == 1 {
                write!(f, " mol")?;
            } else {
                write!(f, " mol^{}", self.unit.mol)?;
            }
        }
        if self.unit.candela != 0 {
            if self.unit.candela == 1 {
                write!(f, " cd")?;
            } else {
                write!(f, " cd^{}", self.unit.candela)?;
            }
        }

        return Ok(());
    }
}

impl<V: F64RealOrComplex> Mul for DynQuantity<V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.mul_assign(rhs);
        return self;
    }
}

impl<V: F64RealOrComplex> Mul<f64> for DynQuantity<V> {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.mul_assign(rhs);
        return self;
    }
}

impl<V: F64RealOrComplex> MulAssign for DynQuantity<V> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        self.unit *= rhs.unit;
    }
}

impl<V: F64RealOrComplex> MulAssign<f64> for DynQuantity<V> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl<V: F64RealOrComplex> Div for DynQuantity<V> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.div_assign(rhs);
        return self;
    }
}

impl<V: F64RealOrComplex> Div<f64> for DynQuantity<V> {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.div_assign(rhs);
        return self;
    }
}

impl<V: F64RealOrComplex> DivAssign for DynQuantity<V> {
    fn div_assign(&mut self, rhs: Self) {
        if self.value.is_infinite() {
            let mut value = self.value / rhs.value;
            if value.re().is_nan() {
                value.set_re_f64(0.0);
            }
            if value.im().is_nan() {
                value.set_im_f64(0.0);
            }
            self.value = value;
        } else {
            self.value /= rhs.value;
        }
        self.unit /= rhs.unit;
    }
}

impl<V: F64RealOrComplex> DivAssign<f64> for DynQuantity<V> {
    fn div_assign(&mut self, rhs: f64) {
        if self.value.is_infinite() {
            let mut value: V = self.value / rhs;
            if value.re().is_nan() {
                value.set_re_f64(0.0);
            }
            if value.im().is_nan() {
                value.set_im_f64(0.0);
            }
            self.value = value;
        } else {
            self.value /= rhs;
        }
    }
}

impl TryFrom<DynQuantity<Complex<f64>>> for DynQuantity<f64> {
    type Error = NotConvertibleFromComplexF64;

    fn try_from(quantity: DynQuantity<Complex<f64>>) -> Result<Self, Self::Error> {
        if quantity.value.im() == 0.0 {
            return Ok(DynQuantity::new(quantity.value.re(), quantity.unit));
        } else {
            return Err(NotConvertibleFromComplexF64 {
                source: quantity.value,
                target_type: "f64",
            });
        }
    }
}

impl From<f64> for DynQuantity<f64> {
    fn from(value: f64) -> Self {
        return DynQuantity::new(value, Default::default());
    }
}

impl From<Complex<f64>> for DynQuantity<Complex<f64>> {
    fn from(value: Complex<f64>) -> Self {
        return DynQuantity::new(value, Default::default());
    }
}

impl TryFrom<DynQuantity<f64>> for f64 {
    type Error = ConversionError;

    fn try_from(quantity: DynQuantity<f64>) -> Result<Self, Self::Error> {
        // If the unit is dimensionless, the conversion succeeds, otherwise it fails
        if quantity.unit.is_dimensionless() {
            return Ok(quantity.value);
        } else {
            return Err(ConversionError::UnitMismatch {
                expected: Unit::default(),
                found: quantity.unit,
            });
        }
    }
}

impl TryFrom<DynQuantity<Complex<f64>>> for Complex<f64> {
    type Error = ConversionError;

    fn try_from(quantity: DynQuantity<Complex<f64>>) -> Result<Self, Self::Error> {
        // If the unit is dimensionless, the conversion succeeds, otherwise it fails
        if quantity.unit.is_dimensionless() {
            return Ok(quantity.value);
        } else {
            return Err(ConversionError::UnitMismatch {
                expected: Unit::default(),
                found: quantity.unit,
            });
        }
    }
}

// ========================================================

/**
Converts a slice of [`DynQuantity`] to a vector of their values `V`. In
constrast to [`to_vec_checked`], this function does not check whether the units
of the individual [`DynQuantity`] elements are identical.
*/
pub fn to_vec<V: F64RealOrComplex>(quantity_slice: &[DynQuantity<V>]) -> Vec<V> {
    let mut output: Vec<V> = Vec::with_capacity(quantity_slice.len());
    for element in quantity_slice.iter() {
        output.push(element.value)
    }
    return output;
}

/**
Checks if all units of the [`DynQuantity`] elements are identical. If that is
the case, it converts the slice to a vector of their values `V`.
*/
pub fn to_vec_checked(quantity_slice: &[DynQuantity<f64>]) -> Result<Vec<f64>, ConversionError> {
    let mut output: Vec<f64> = Vec::with_capacity(quantity_slice.len());
    if let Some(first_element) = quantity_slice.first() {
        let first_elem_unit = first_element.unit;
        for element in quantity_slice.iter() {
            if element.unit != first_elem_unit {
                return Err(ConversionError::UnitMismatch {
                    expected: first_elem_unit,
                    found: element.unit,
                });
            }
            output.push(element.value)
        }
    }
    return Ok(output);
}
