/*!
This module is only available if the [`serde`] feature is enabled.
It provides various functions which can be used to (fallible)
deserialize a valid [`DynQuantity`] representation into any type `T` which
implements [`TryFrom<DynQuantity>`]. See the docstring of [`DynQuantity`] for an
overview over all possible representations.
*/

use std::{marker::PhantomData, str::FromStr};

use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use num::Complex;
use serde::{Deserialize, Deserializer, de::DeserializeOwned};

use crate::{ConversionError, DynQuantity, UnitExponents};

#[derive(DeserializeUntaggedVerboseError)]
enum NumberOrString<T> {
    Number(T),
    String(String),
}

/**
Deserializes a type `T` implementing [`TryFrom<DynQuantity>`] from a valid
[`DynQuantity`] representation (see docstring of [`DynQuantity`]).

This function is meant to be used in conjunction with [`serde`]s `Deserialize`
macro and the `deserialize_with` annotation:

# Examples
```
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
 */
pub fn deserialize_quantity<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: DeserializeOwned + TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
{
    match deserialize_opt_quantity(deserializer)? {
        Some(quantity) => Ok(quantity),
        None => Err(serde::de::Error::custom("expected a quantity, found none")),
    }
}

/**
Like [`deserialize_quantity`], but deserializes into an [`Option<Quantity>`]
instead of a [`Quantity`].

# Examples
```
use serde::{Deserialize};
use uom::si::{f64::Length, length::meter};
use dyn_quantity::deserialize_opt_quantity;
use indoc::indoc;

#[derive(Deserialize, Debug)]
struct OptLengthWrapper {
    #[serde(deserialize_with = "deserialize_opt_quantity")]
    opt_length: Option<Length>,
}

let ser = indoc! {"
---
opt_length: 1200 mm
"};
let wrapper: OptLengthWrapper = serde_yaml::from_str(&ser).unwrap();
assert_eq!(wrapper.opt_length.unwrap().get::<meter>(), 1.2);

let ser = indoc! {"
---
opt_length:
"};
let wrapper: OptLengthWrapper = serde_yaml::from_str(&ser).unwrap();
assert!(wrapper.opt_length.is_none());
```
 */
pub fn deserialize_opt_quantity<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: DeserializeOwned + TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
{
    let quantity = Option::<NumberOrString<_>>::deserialize(deserializer)?;
    match quantity {
        Some(number_or_string) => {
            match number_or_string {
                NumberOrString::Number(quantity) => Ok(Some(quantity)),
                NumberOrString::String(string) => {
                    // Deserialize using the SI unit parser
                    let quantity = DynQuantity::<Complex<f64>>::from_str(&string)
                        .map_err(serde::de::Error::custom)?;
                    T::try_from(quantity)
                        .map_err(serde::de::Error::custom)
                        .map(Some)
                }
            }
        }
        None => Ok(None),
    }
}

/**
Deserializes an angle from a valid [`DynQuantity`] representation (see
docstring of [`DynQuantity`]). The output value is always in radians.

This function is meant to be used in conjunction with [`serde`]s `Deserialize`
macro and the `deserialize_with` annotation:

# Examples
```
use serde::{Deserialize};
use dyn_quantity::deserialize_angle;
use indoc::indoc;
use std::f64::consts::PI;

#[derive(Deserialize, Debug)]
struct AngleWrapper {
    #[serde(deserialize_with = "deserialize_angle")]
    angle: f64,
}

let ser = indoc! {"
---
angle: 360 / 2 degree
"};
let wrapper: AngleWrapper = serde_yaml::from_str(&ser).unwrap();
assert_eq!(wrapper.angle, PI);
```
 */
pub fn deserialize_angle<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match deserialize_opt_angle(deserializer)? {
        Some(angle) => Ok(angle),
        None => Err(serde::de::Error::custom("expected an angle, found none")),
    }
}

/**
Like [`deserialize_angle`], but deserializes into an [`Option<f64>`]
instead of a [`f64`].

# Examples
```
use serde::{Deserialize};
use dyn_quantity::deserialize_opt_angle;
use indoc::indoc;

#[derive(Deserialize, Debug)]
struct OptAngleWrapper {
    #[serde(deserialize_with = "deserialize_opt_angle")]
    opt_angle: Option<f64>,
}

let ser = indoc! {"
---
opt_angle: 2 rad
"};
let wrapper: OptAngleWrapper = serde_yaml::from_str(&ser).unwrap();
assert_eq!(wrapper.opt_angle.unwrap(), 2.0);

let ser = indoc! {"
---
opt_angle:
"};
let wrapper: OptAngleWrapper = serde_yaml::from_str(&ser).unwrap();
assert!(wrapper.opt_angle.is_none());
```
 */
pub fn deserialize_opt_angle<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let quantity = Option::<NumberOrString<f64>>::deserialize(deserializer)?;
    match quantity {
        Some(number_or_string) => {
            match number_or_string {
                NumberOrString::Number(quantity) => Ok(Some(quantity)),
                NumberOrString::String(string) => {
                    // Deserialize using the SI unit parser
                    let quantity =
                        DynQuantity::<f64>::from_str(&string).map_err(serde::de::Error::custom)?;
                    return Ok(Some(quantity.value));
                }
            }
        }
        None => Ok(None),
    }
}

/**
Deserializes a vector of `T` which implements [`TryFrom<DynQuantity>`] from:

1) A vector representation of [`DynQuantity`] or
2) A string representing a vector of numbers with a unit behind it.

# Examples:
```
use indoc::indoc;
use serde::{Deserialize};
use uom::si::{f64::Length, length::meter};
use dyn_quantity::deserialize_vec_of_quantities;

#[derive(Deserialize, Debug)]
struct VecWrapper {
    #[serde(deserialize_with = "deserialize_vec_of_quantities")]
    vec: Vec<Length>,
}

// Variant 1: Vector representation of `DynQuantity`
let ser = indoc! {"
---
vec: [1 m, 2 mm, 3 km]
"};
let wrapper: VecWrapper = serde_yaml::from_str(&ser).unwrap();
assert_eq!(wrapper.vec[0].get::<meter>(), 1.0);
assert_eq!(wrapper.vec[1].get::<meter>(), 0.002);
assert_eq!(wrapper.vec[2].get::<meter>(), 3000.0);

// Variant 2: Vector of numbers with unit at the end
let ser = indoc! {"
---
vec: '[1, 2e-3, 3e3] m'
"};
let wrapper: VecWrapper = serde_yaml::from_str(&ser).unwrap();
assert_eq!(wrapper.vec[0].get::<meter>(), 1.0);
assert_eq!(wrapper.vec[1].get::<meter>(), 0.002);
assert_eq!(wrapper.vec[2].get::<meter>(), 3000.0);
```
 */
pub fn deserialize_vec_of_quantities<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: DeserializeOwned + TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
{
    match deserialize_opt_vec_of_quantities(deserializer)? {
        Some(vec) => Ok(vec),
        None => Err(serde::de::Error::custom("expected a vector, found none")),
    }
}

/**
Like [`deserialize_vec_of_quantities`], but deserializes into an [`Option<Vec<T>>`]
instead of a [`Vec<T>`].

# Examples:
```
use indoc::indoc;
use serde::{Deserialize};
use uom::si::{f64::Length, length::meter};
use dyn_quantity::deserialize_opt_vec_of_quantities;

#[derive(Deserialize, Debug)]
struct OptVecWrapper {
    #[serde(deserialize_with = "deserialize_opt_vec_of_quantities")]
    vec: Option<Vec<Length>>,
}

// Vector given (variant 1)
let ser = indoc! {"
---
vec: [1 m, 2 mm, 3 km]
"};
let wrapper: OptVecWrapper = serde_yaml::from_str(&ser).unwrap();
let vec = wrapper.vec.unwrap();
assert_eq!(vec[0].get::<meter>(), 1.0);
assert_eq!(vec[1].get::<meter>(), 0.002);
assert_eq!(vec[2].get::<meter>(), 3000.0);

// No vector given
let ser = indoc! {"
---
vec:
"};
let wrapper: OptVecWrapper = serde_yaml::from_str(&ser).unwrap();
assert!(wrapper.vec.is_none());
```
*/
pub fn deserialize_opt_vec_of_quantities<'de, D, T>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: DeserializeOwned + TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
{
    match Option::<QuantityVecEnum<T>>::deserialize(deserializer)? {
        Some(q) => {
            match q {
                QuantityVecEnum::Vec(vec) => return Ok(Some(vec)),
                QuantityVecEnum::QuantityVec(vec) => return Ok(Some(vec.0)),
                QuantityVecEnum::String(string) => {
                    // Remove the unit from the string by finding the closing
                    // bracket of the vector "]". The slice before the closing
                    // bracket can then be deserialized as a vector of floats,
                    // while the slice behind the closing bracket can be
                    // interpreted as UnitExponents.
                    match string.find(']') {
                        Some(byte) => {
                            if let Some(quantity_str) = string.get(byte + 1..) {
                                let quantity = DynQuantity::from_str(quantity_str)
                                    .map_err(serde::de::Error::custom)?;

                                let vec_str = string.get(..(byte + 1)).expect("must not be empty");
                                return parse_vec(vec_str, &quantity)
                                    .map(Some)
                                    .map_err(serde::de::Error::custom);
                            } else {
                                return parse_vec(
                                    &string,
                                    &DynQuantity::new(
                                        Complex::new(1.0, 0.0),
                                        UnitExponents::default(),
                                    ),
                                )
                                .map(Some)
                                .map_err(serde::de::Error::custom);
                            };
                        }
                        None => {
                            return Err(serde::de::Error::custom(
                                "expected a vector, but did not find the closing bracket ]",
                            ));
                        }
                    }
                }
            }
        }
        None => return Ok(None),
    }
}

fn parse_vec<T, E>(input: &str, multiplier: &DynQuantity<Complex<f64>>) -> Result<Vec<T>, String>
where
    T: TryFrom<DynQuantity<Complex<f64>>, Error = E>,
    E: std::fmt::Display,
{
    let mut output = Vec::new();
    for slice in input.split(&['[', ',', ']'][..]) {
        // Skip slices which are empty or contain
        if slice.is_empty() {
            continue;
        }
        let quantity = match DynQuantity::from_str(slice) {
            Ok(quantity) => quantity,
            Err(_) => continue,
        };
        let quantity_mult = quantity * multiplier.clone();
        let element: T = quantity_mult.try_into().map_err(|err: E| err.to_string())?;
        output.push(element)
    }
    return Ok(output);
}

#[derive(DeserializeUntaggedVerboseError)]
enum QuantityVecEnum<T>
where
    T: TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
{
    Vec(Vec<T>),
    QuantityVec(QuantityVec<T>),
    String(String),
}

struct QuantityVec<T>(Vec<T>)
where
    T: TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display;

impl<'de, T> Deserialize<'de> for QuantityVec<T>
where
    T: serde::de::Deserialize<'de> + TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor<T> {
            marker: PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: serde::de::Deserialize<'de> + TryFrom<DynQuantity<Complex<f64>>>,
            <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
        {
            type Value = QuantityVec<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec: QuantityVec<T> = match seq.size_hint() {
                    Some(capacity) => QuantityVec(Vec::with_capacity(capacity)),
                    None => QuantityVec(Vec::new()),
                };

                let first_value = match seq.next_element::<NumberOrString<T>>()? {
                    Some(element) => element,
                    None => return Ok(vec), // Empty vec
                };

                match first_value {
                    NumberOrString::Number(number) => {
                        vec.0.push(number);
                        while let Some(quantity_rep) = seq.next_element::<NumberOrString<T>>()? {
                            match quantity_rep {
                                NumberOrString::Number(quantity) => vec.0.push(quantity),
                                NumberOrString::String(_) => {
                                    return Err(serde::de::Error::custom(
                                        "either all elements of the vector must have the same quantity, or no element must have a quantity",
                                    ));
                                }
                            }
                        }
                    }
                    NumberOrString::String(string) => {
                        let first_element =
                            DynQuantity::from_str(&string).map_err(serde::de::Error::custom)?;
                        let first_element_unit = first_element.exponents.clone();
                        let output_element =
                            first_element.try_into().map_err(serde::de::Error::custom)?;
                        vec.0.push(output_element);

                        while let Some(quantity_rep) = seq.next_element::<NumberOrString<T>>()? {
                            // Loop through all other elements and check if their unit is equal to first_element_unit
                            match quantity_rep {
                                NumberOrString::Number(_) => {
                                    return Err(serde::de::Error::custom(
                                        "either all elements of the vector must have the same quantity, or no element must have a quantity",
                                    ));
                                }
                                NumberOrString::String(string) => {
                                    let element = DynQuantity::<Complex<f64>>::from_str(&string)
                                        .map_err(serde::de::Error::custom)?;
                                    if element.exponents != first_element_unit {
                                        return Err(serde::de::Error::custom(
                                            ConversionError::UnitMismatch {
                                                expected: first_element_unit,
                                                found: element.exponents,
                                            },
                                        ));
                                    }
                                    let output_element =
                                        element.try_into().map_err(serde::de::Error::custom)?;
                                    vec.0.push(output_element);
                                }
                            }
                        }
                    }
                }

                Ok(vec)
            }
        }

        let visitor = Visitor {
            marker: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_number_or_string() {
        {
            let value: NumberOrString<DynQuantity<f64>> = serde_yaml::from_str("1.0").unwrap();
            match value {
                NumberOrString::Number(value) => {
                    assert_eq!(value.value, 1.0);
                }
                NumberOrString::String(_) => unreachable!(),
            }
        }
        {
            let value: NumberOrString<DynQuantity<f64>> = serde_yaml::from_str("1.0 A").unwrap();
            match value {
                NumberOrString::Number(value) => {
                    assert_eq!(value.value, 1.0);
                    assert_eq!(value.exponents.ampere, 1);
                }
                NumberOrString::String(_) => unreachable!(),
            }
        }
        {
            let value: NumberOrString<DynQuantity<Complex<f64>>> =
                serde_yaml::from_str("1.0 A").unwrap();
            match value {
                NumberOrString::Number(value) => {
                    assert_eq!(value.value.re, 1.0);
                    assert_eq!(value.value.im, 0.0);
                    assert_eq!(value.exponents.ampere, 1);
                }
                NumberOrString::String(_) => unreachable!(),
            }
        }
    }
}
