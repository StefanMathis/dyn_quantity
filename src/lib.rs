#![doc = include_str!("../README.md")]

#[cfg(feature = "uom")]
pub mod uom_impl;

#[cfg(feature = "serde")]
pub mod serde_impl;

#[cfg(feature = "serde")]
pub mod deserialize_with;

#[cfg(feature = "serde")]
pub use deserialize_with::{
    deserialize_angle, deserialize_opt_angle, deserialize_opt_quantity,
    deserialize_opt_vec_of_quantities, deserialize_quantity, deserialize_vec_of_quantities,
};

// =============================================================================
// From here, the code needs to be copied into dyn_quantity/dyn_quantity_from_str/src/lib.rs.

pub mod from_str;

use num::Complex;
use num::complex::ComplexFloat;

use std::error::Error;
use std::fmt::Display;
use std::ops::{Div, DivAssign, Mul, MulAssign};
pub use std::str::FromStr;

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
    + std::ops::MulAssign
    + std::ops::DivAssign
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
type system, but rather via the values of the [`UnitExponents`]. This means that
the unit of measurement is not fixed at compile time, but can change dynamically
at runtime.

This property is very useful when e.g. parsing a user-provided string to a
physical quantity. For this case, [`DynQuantity`] implements the
[`FromStr`](`std::str::FromStr`) trait. The module documentation [`from_str`]
has more information regarding the available syntax.

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

If the [`uom`] feature is enabled, a [`DynQuantity`] can be (fallible)
converted into a [`uom::si::Quantity`] via [`TryFrom`]. In combination with the
aforementioned parsing capabilities, this allows fallible parsing of strings to
statically-typed physical quantities.

# Serialization and deserialization

If the [`serde`] feature is enabled, this struct can be serialized and
deserialized. Serialization creates the "standard" [`serde`] representation
one would expect from the [`Serialize`] macro. When deserializing however,
multiple options are available:
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
implementation under the hood, see the [`from_str`] module documentation.
3) Deserialize directly from a real or complex value. This option is mainly here
to allow deserializing a serialized [`uom`] quantity (whose serialized
representation is simply its numerical value without any units). For example,
deserializing `5.0` into [`DynQuantity<f64>`] produces the same result as
deserializing:
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
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct DynQuantity<V: F64RealOrComplex> {
    /**
    The value of the physical quantity.
     */
    pub value: V,
    /**
    The (SI) base units of the physical quantity, represented by their exponents.
     */
    pub exponents: UnitExponents,
}

impl<V: F64RealOrComplex> DynQuantity<V> {
    /**
    Returns a new instance of `Self`.
     */
    pub fn new(value: V, exponents: UnitExponents) -> Self {
        return Self { value, exponents };
    }

    /**
    Fallible addition of `self` and `other`.

    Physical quantities can only be added together if their units are identical.
    Hence, this function first compares the `.exponents` fields of `self` and
    `other`. If they are identical, the `value` fields are added up and the
    resulting quantity is returned. Otherwise, a [`UnitsOfSummandsNotIdentical`]
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
    assert_eq!(curr_sum.exponents.ampere, 1);

    // ... but adding a current to a voltage fails.
    assert!(volt1.try_add(&curr1).is_err());
    ```
     */
    pub fn try_add(&self, other: &Self) -> Result<Self, UnitsOfSummandsNotIdentical> {
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
    assert_eq!(curr1.exponents.ampere, 1);

    // volt1 does not get modified because the addition failed
    let volt_cpy = volt1.clone();
    assert!(volt1.try_add_assign(&curr1).is_err());
    assert_eq!(volt1, volt_cpy);
    ```
     */
    pub fn try_add_assign(&mut self, other: &Self) -> Result<(), UnitsOfSummandsNotIdentical> {
        if self.exponents == other.exponents {
            self.value += other.value;
            return Ok(());
        } else {
            return Err(UnitsOfSummandsNotIdentical(
                self.exponents.clone(),
                other.exponents.clone(),
            ));
        }
    }

    /**
    Fallible subtraction of `self` and `other`.

    Physical quantities can only be subtracted from each other if their units are identical.
    Hence, this function first compares the `.exponents` fields of `self` and
    `other`. If they are identical, the `value` fields are subtracted up and the
    resulting quantity is returned. Otherwise, a [`UnitsOfSummandsNotIdentical`]
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
    assert_eq!(curr_sum.exponents.ampere, 1);

    // ... but sbutracting a current from a voltage fails.
    assert!(volt1.try_sub(&curr1).is_err());
    ```
     */
    pub fn try_sub(&self, other: &Self) -> Result<Self, UnitsOfSummandsNotIdentical> {
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
    assert_eq!(curr1.exponents.ampere, 1);

    // volt1 does not get modified because the addition failed
    let volt_cpy = volt1.clone();
    assert!(volt1.try_sub_assign(&curr1).is_err());
    assert_eq!(volt1, volt_cpy);
    ```
     */
    pub fn try_sub_assign(&mut self, other: &Self) -> Result<(), UnitsOfSummandsNotIdentical> {
        if self.exponents == other.exponents {
            self.value -= other.value;
            return Ok(());
        } else {
            return Err(UnitsOfSummandsNotIdentical(
                self.exponents.clone(),
                other.exponents.clone(),
            ));
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
    assert_eq!(result.exponents.ampere, 6);
    ```
     */
    pub fn powi(mut self, n: i32) -> Self {
        self.value = self.value.powi(n);
        self.exponents = self.exponents.powi(n);
        return self;
    }

    /**
    Tries to calculate the `n`th root of self.
     */
    pub fn try_nthroot(mut self, n: i32) -> Result<Self, RootError> {
        self.exponents = self.exponents.try_nthroot(n)?;
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
        if self.exponents.second != 0 {
            if self.exponents.second == 1 {
                write!(f, " s")?;
            } else {
                write!(f, " s^{}", self.exponents.second)?;
            }
        }
        if self.exponents.meter != 0 {
            if self.exponents.meter == 1 {
                write!(f, " m")?;
            } else {
                write!(f, " m^{}", self.exponents.meter)?;
            }
        }
        if self.exponents.kilogram != 0 {
            if self.exponents.kilogram == 1 {
                write!(f, " kg")?;
            } else {
                write!(f, " kg^{}", self.exponents.kilogram)?;
            }
        }
        if self.exponents.ampere != 0 {
            if self.exponents.ampere == 1 {
                write!(f, " A")?;
            } else {
                write!(f, " A^{}", self.exponents.ampere)?;
            }
        }
        if self.exponents.kelvin != 0 {
            if self.exponents.kelvin == 1 {
                write!(f, " K")?;
            } else {
                write!(f, " K^{}", self.exponents.kelvin)?;
            }
        }
        if self.exponents.mol != 0 {
            if self.exponents.mol == 1 {
                write!(f, " mol")?;
            } else {
                write!(f, " mol^{}", self.exponents.mol)?;
            }
        }
        if self.exponents.candela != 0 {
            if self.exponents.candela == 1 {
                write!(f, " cd")?;
            } else {
                write!(f, " cd^{}", self.exponents.candela)?;
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

impl<V: F64RealOrComplex> MulAssign for DynQuantity<V> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        self.exponents *= rhs.exponents;
    }
}

impl<V: F64RealOrComplex> Div for DynQuantity<V> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
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
        self.exponents /= rhs.exponents;
    }
}

impl TryFrom<DynQuantity<Complex<f64>>> for DynQuantity<f64> {
    type Error = NotConvertibleFromComplexF64;

    fn try_from(quantity: DynQuantity<Complex<f64>>) -> Result<Self, Self::Error> {
        if quantity.value.im() == 0.0 {
            return Ok(DynQuantity::new(quantity.value.re(), quantity.exponents));
        } else {
            return Err(NotConvertibleFromComplexF64 {
                source: quantity.value,
                target_type: "f64",
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
        let exponents = first_element.exponents.clone();
        for element in quantity_slice.iter() {
            if element.exponents != exponents {
                return Err(ConversionError::UnitMismatch {
                    expected: exponents,
                    found: element.exponents.clone(),
                });
            }
            output.push(element.value)
        }
    }
    return Ok(output);
}

// ====================================================

/**
Struct representing a unit of measurement in the SI system via the exponents of
the base units.
 */
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct UnitExponents {
    pub second: i32,
    pub meter: i32,
    pub kilogram: i32,
    pub ampere: i32,
    pub kelvin: i32,
    pub mol: i32,
    pub candela: i32,
}

impl From<[i32; 7]> for UnitExponents {
    /**
    Converts an array of seven `i32` values into `UnitExponents`.

    The individual array elements are interpreted as follows:
    - `array[0]`: Exponent of second
    - `array[1]`: Exponent of meter
    - `array[2]`: Exponent of kilogram
    - `array[3]`: Exponent of ampere
    - `array[4]`: Exponent of kelvin
    - `array[5]`: Exponent of mol
    - `array[6]`: Exponent of candela
     */
    fn from(array: [i32; 7]) -> Self {
        return UnitExponents {
            second: array[0],
            meter: array[1],
            kilogram: array[2],
            ampere: array[3],
            kelvin: array[4],
            mol: array[5],
            candela: array[6],
        };
    }
}

impl From<UnitExponents> for [i32; 7] {
    /**
    Converts an `UnitExponents` into an array of seven `i32`.

    The exponents are put into the array in the following order:
    - `array[0]`: Exponent of second
    - `array[1]`: Exponent of meter
    - `array[2]`: Exponent of kilogram
    - `array[3]`: Exponent of ampere
    - `array[4]`: Exponent of kelvin
    - `array[5]`: Exponent of mol
    - `array[6]`: Exponent of candela
     */
    fn from(value: UnitExponents) -> Self {
        return [
            value.second,
            value.meter,
            value.kilogram,
            value.ampere,
            value.kelvin,
            value.mol,
            value.candela,
        ];
    }
}

impl UnitExponents {
    /**
    Raises `self` to an integer power.

    # Examples
    ```
    use dyn_quantity::UnitExponents;

    let exponents = UnitExponents::from([0, 1, 0, 2, 0, -2, 0]);
    let array: [i32; 7] = exponents.powi(2).into();
    assert_eq!(array, [0, 2, 0, 4, 0, -4, 0]);
    ```
     */
    pub fn powi(mut self, n: i32) -> Self {
        self.second *= n;
        self.meter *= n;
        self.kilogram *= n;
        self.ampere *= n;
        self.kelvin *= n;
        self.mol *= n;
        self.candela *= n;
        return self;
    }

    /**
    Tries to calculate the `n`th root of self. This operation fails if any
    of the exponents is not divisible by `n`.

    # Examples
    ```
    use dyn_quantity::UnitExponents;

    let exponents = UnitExponents::from([0, 2, 0, 2, 0, -4, 0]);

    // It is possible to calculate the square root:
    let array: [i32; 7] = exponents.clone().try_nthroot(2).unwrap().into();
    assert_eq!(array, [0, 1, 0, 1, 0, -2, 0]);

    // But not the cubic root (not all exponents are divisible by 3):
    assert!(exponents.try_nthroot(3).is_err());
    ```
     */
    pub fn try_nthroot(mut self, n: i32) -> Result<Self, RootError> {
        fn try_nthroot_inner(
            exponents: &UnitExponents,
            exp: i32,
            n: i32,
        ) -> Result<i32, RootError> {
            if exp % n == 0 {
                return Ok(exp / n);
            } else {
                return Err(RootError {
                    n,
                    exponents: exponents.clone(),
                });
            }
        }
        let init_exp = self.clone();
        self.second = try_nthroot_inner(&init_exp, self.second, n)?;
        self.meter = try_nthroot_inner(&init_exp, self.meter, n)?;
        self.kilogram = try_nthroot_inner(&init_exp, self.kilogram, n)?;
        self.ampere = try_nthroot_inner(&init_exp, self.ampere, n)?;
        self.kelvin = try_nthroot_inner(&init_exp, self.kelvin, n)?;
        self.mol = try_nthroot_inner(&init_exp, self.mol, n)?;
        self.candela = try_nthroot_inner(&init_exp, self.candela, n)?;
        return Ok(self);
    }
}

impl std::fmt::Display for UnitExponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "s^{} m^{} kg^{} A^{} K^{} mol^{} cd^{}",
            self.second,
            self.meter,
            self.kilogram,
            self.ampere,
            self.kelvin,
            self.mol,
            self.candela
        )
    }
}

impl Mul for UnitExponents {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.mul_assign(rhs);
        return self;
    }
}

impl MulAssign for UnitExponents {
    fn mul_assign(&mut self, rhs: Self) {
        self.second += rhs.second;
        self.meter += rhs.meter;
        self.kilogram += rhs.kilogram;
        self.ampere += rhs.ampere;
        self.kelvin += rhs.kelvin;
        self.mol += rhs.mol;
        self.candela += rhs.candela;
    }
}

impl Div for UnitExponents {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.div_assign(rhs);
        return self;
    }
}

impl DivAssign for UnitExponents {
    fn div_assign(&mut self, rhs: Self) {
        self.second -= rhs.second;
        self.meter -= rhs.meter;
        self.kilogram -= rhs.kilogram;
        self.ampere -= rhs.ampere;
        self.kelvin -= rhs.kelvin;
        self.mol -= rhs.mol;
        self.candela -= rhs.candela;
    }
}

/**
Error representing a failed attempt to add two physical quantities.

Two physical quantities can only be added if their units of measurements are
identical. If they aren't, the corresponding function will return an instance
of this struct, which holds the [`UnitExponents`] of both quantities for
further inspection.
 */
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct UnitsOfSummandsNotIdentical(pub UnitExponents, pub UnitExponents);

impl Display for UnitsOfSummandsNotIdentical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "first summand has exponents {}, but second summand has exponents {}",
            self.0, self.1
        )
    }
}

impl Error for UnitsOfSummandsNotIdentical {}

/**
Error representing a failed attempt to calculate the `n`th root of a [`UnitExponents`].

Calculating the `n`th root of a [`UnitExponents`] fails if any of the exponents
is not divisible by `n`.
 */
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RootError {
    pub n: i32,
    pub exponents: UnitExponents,
}

impl std::fmt::Display for RootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "not possible to calculate the {}th root (exponents {} cannot be divided by {} without remainder)",
            &self.n, &self.exponents, &self.n
        )
    }
}

impl std::error::Error for RootError {}

/**
Error representing a failed attempt to parse a string into a [`DynQuantity`].
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
The varying reasons parsing a string to a [`DynQuantity`] can fail.
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
    /// combined in the resulting [`DynQuantity`].
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
    UnitsOfSummandsNotIdentical(UnitsOfSummandsNotIdentical),
    /// See docstring of [`NotConvertibleFromComplexF64`].
    NotConvertibleFromComplexF64(NotConvertibleFromComplexF64),
    /// Generic fallback error for all other parsing failures
    #[default]
    CouldNotParse,
}

impl ParseErrorReason {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
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
            ParseErrorReason::UnitsOfSummandsNotIdentical(inner) => inner.fmt(f),
            ParseErrorReason::MustNotStartWith => {
                write!(f, "input must not start with this token")
            }
            ParseErrorReason::NotConvertibleFromComplexF64(err) => err.fmt(f),
        }
    }
}

impl From<UnitsOfSummandsNotIdentical> for ParseErrorReason {
    fn from(value: UnitsOfSummandsNotIdentical) -> Self {
        return Self::UnitsOfSummandsNotIdentical(value);
    }
}

impl std::error::Error for ParseErrorReason {}

/**
Error describing a failed attempt to convert a [`Complex<f64>`] into the type
`V` of [`DynQuantity<V>`].

For example, this error will be returned when trying to parse a string
representing a complex quantity into a [`DynQuantity<f64>`].
 */
#[derive(Debug, Clone, PartialEq)]
pub struct NotConvertibleFromComplexF64 {
    pub source: Complex<f64>,
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

This error can e.g. be returned when trying to convert a [`DynQuantity`] to a
[`uom::si::Quantity`] via the [`TryFrom`] implementation. See docstring of
[`DynQuantity`].
*/
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    /// See docstring of [`NotConvertibleFromComplexF64`].
    NotConvertibleFromComplexF64(NotConvertibleFromComplexF64),
    /// Expected a certain unit of measurement, but found a different one.
    UnitMismatch {
        expected: UnitExponents,
        found: UnitExponents,
    },
    /// Fallback case for all other errors.
    Custom(String),
}

impl ConversionError {
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
