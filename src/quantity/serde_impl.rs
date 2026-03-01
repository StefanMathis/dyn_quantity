/*!
This module is only available if the [`serde`] feature is enabled.
It provides various functions which can be used to (fallible)
deserialize a valid [`DynQuantity`] representation into any type `T` which
implements [`TryFrom<DynQuantity>`]. See the docstring of [`DynQuantity`] for an
overview over all possible representations.
*/

use std::cell::Cell;
#[cfg(feature = "from_str")]
use std::str::FromStr;

use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::F64RealOrComplex;
use crate::error::{ConversionError, ParseError};
use crate::unit::Unit;

impl<V> Serialize for DynQuantity<V>
where
    V: F64RealOrComplex + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DynQuantity", 2)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("unit", &self.unit)?;
        state.end()
    }
}

impl<'de, V> Deserialize<'de> for DynQuantity<V>
where
    V: F64RealOrComplex + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<DynQuantity<V>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variants = QuantityVariants::<V>::deserialize(deserializer)?;
        variants.try_into().map_err(serde::de::Error::custom)
    }
}

/**
A [`DynQuantity`] can be deserialized a couple of different representations.
 */
#[derive(DeserializeUntaggedVerboseError)]
enum QuantityVariants<V>
where
    V: F64RealOrComplex,
{
    /**
    Native representation of [`DynQuantity`] (via an alias struct in order
    to avoid infinite recursion)-
     */
    Quantity(QuantityAlias<V>),
    /**
    String representation using the [`std::str::FromStr`] implementation for
    [`DynQuantity`].

    Only available if the `from_str` feature is enabled.
     */
    #[cfg(feature = "from_str")]
    String(String),
    /**
    A value without any units - in that case, the unit exponents are assumed
    to be zero and the value to be dimensionless.
     */
    Value(V),
}

#[derive(serde::Deserialize)]
struct QuantityAlias<V: F64RealOrComplex> {
    value: V,
    unit: Unit,
}

impl<V: F64RealOrComplex> TryFrom<QuantityVariants<V>> for DynQuantity<V> {
    type Error = ParseError;

    fn try_from(variant: QuantityVariants<V>) -> Result<Self, Self::Error> {
        match variant {
            QuantityVariants::Quantity(variant) => {
                return Ok(Self {
                    value: variant.value,
                    unit: variant.unit,
                });
            }
            #[cfg(feature = "from_str")]
            QuantityVariants::String(string) => {
                return Self::from_str(&string);
            }
            QuantityVariants::Value(value) => {
                return Ok(Self {
                    value,
                    unit: Unit::default(),
                });
            }
        }
    }
}

use std::marker::PhantomData;

use crate::DynQuantity;
use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use num::Complex;
use serde::{Deserialize, Deserializer, de::DeserializeOwned};

#[derive(DeserializeUntaggedVerboseError)]
enum InnerOrString<T> {
    Inner(T),
    #[cfg(feature = "from_str")]
    String(String),
}

thread_local!(
    /**
    A thread-local, static variable which enables / disables serialization of
    quantities with or without units. It is used within the functions
    [`serialize_quantity`], [`serialize_opt_quantity`], [`serialize_angle`] and
    [`serialize_opt_angle`] as a thread-local context to decide whether a
    quantity should be serialized with or without its units. By default, its
    value is `false`, meaning that quantities are serialized without their
    units. The [`serialize_with_units`] function sets it temporarily to `true`,
    then performs the actual serialization, and afterwards resets it to `false`
    again (return to default behaviour).

    Direct interaction with this variable is only necessary when writing a
    multithreaded serializer, see the docstring of [`serialize_with_units`].
    Otherwise, it is highly recommended to leave this variable alone.
    */
    pub static SERIALIZE_WITH_UNITS: Cell<bool> = Cell::new(false)
);

/**
A wrapper around a serialization function / closure which enables serialization
with units.

# Overview

By default, a quantity or angle is often serialized as its raw value in base SI
units, which is very efficient. Sometimes, it might be useful to store a value
together with its units. If a struct field uses [`serialize_quantity`],
[`serialize_opt_quantity`], [`serialize_angle`] or [`serialize_opt_angle`]
for serialization and this wrapper is applied to the actual serialization
function, then the units are stored together with the raw value in a string.

# Multithreaded serialization

This function is a thin wrapper around the passed in serialization function
which just sets [`SERIALIZE_WITH_UNITS`] to `true`, then calls the passed in
function, stores the result, then sets [`SERIALIZE_WITH_UNITS`] back to `false`
and finally returns the result. Since [`SERIALIZE_WITH_UNITS`] is a thread-local
variable, it will be set to its default value `false` if a new thread is created
inside the passed function. Hence, a multithreaded serializer needs to adjust
the value of [`SERIALIZE_WITH_UNITS`] in each thread it creates - which is why
[`SERIALIZE_WITH_UNITS`] is exposed in the first place.

# Examples

```
use serde::{Serialize};
use uom::si::{f64::Length, length::{millimeter, kilometer}};
use dyn_quantity::*;
use indoc::indoc;

#[derive(Serialize, Debug)]
struct Quantities {
    #[serde(serialize_with = "serialize_quantity")]
    length: Length,
    #[serde(serialize_with = "serialize_opt_quantity")]
    opt_length: Option<Length>,
    #[serde(serialize_with = "serialize_angle")]
    angle: f64,
    #[serde(serialize_with = "serialize_opt_angle")]
    opt_angle: Option<f64>,
}

let quantities = Quantities {
    length: Length::new::<millimeter>(1.0),
    opt_length: Some(Length::new::<kilometer>(1.0)),
    angle: 1.0,
    opt_angle: Some(2.0),
};

// Without units (standard serialization)
let expected = indoc! {"
---
length: 0.001
opt_length: 1000.0
angle: 1.0
opt_angle: 2.0

"};
let actual = serde_yaml::to_string(&quantities).expect("serialization succeeds");
assert_eq!(expected, actual);

// With units
let expected = indoc! {"
---
length: 0.001 m
opt_length: 1000 m
angle: 1 rad
opt_angle: 2 rad

"};
let actual = serialize_with_units(||{serde_yaml::to_string(&quantities)}).expect("serialization succeeds");
assert_eq!(expected, actual);
```
 */
pub fn serialize_with_units<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    SERIALIZE_WITH_UNITS.with(|ctx| {
        ctx.set(true);
        let res = f();
        ctx.set(false);
        res
    })
}

/**
Enables serialization of a quantity (any type implementing
[`Into<DynQuantity>`]) into a string containing both the value and the units.

When a value is serialized using [`serialize_with_units`], this function stores
a quantity as a string containing both the raw value and the units.
If [`serialize_with_units`] is not used, this function serializes its field
using the default [`Serialize`] implementation of the type.

For examples see the [`serialize_with_units`] documentation.
 */
pub fn serialize_quantity<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Serialize,
    for<'a> &'a T: Into<DynQuantity<Complex<f64>>>,
{
    SERIALIZE_WITH_UNITS.with(|ctx| {
        if ctx.get() {
            let quantity: DynQuantity<Complex<f64>> = value.into();
            let string = quantity.to_string();
            string.serialize(serializer)
        } else {
            value.serialize(serializer)
        }
    })
}

/**
Like [`serialize_quantity`], but serializes an [`&Option<T>`]
instead of a `&T` implementing [`Into<DynQuantity>`].

For examples see the [`serialize_with_units`] documentation.
 */
pub fn serialize_opt_quantity<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Serialize,
    for<'a> &'a T: Into<DynQuantity<Complex<f64>>>,
{
    match value.as_ref() {
        Some(v) => {
            let quantity: DynQuantity<Complex<f64>> = v.into();
            SERIALIZE_WITH_UNITS.with(|ctx| {
                if ctx.get() {
                    let string = quantity.to_string();
                    string.serialize(serializer)
                } else {
                    value.serialize(serializer)
                }
            })
        }
        None => return serializer.serialize_none(),
    }
}

/**
Enables serialization of an angle into a string containing both the value and
the "rad" unit.

When a value is serialized using [`serialize_with_units`], this function stores
an angle as a string containing both the raw value and the "rad" unit.
If [`serialize_with_units`] is not used, this function serializes its field
using the default [`Serialize`] implementation of the type.

For examples see the [`serialize_with_units`] documentation.
 */
pub fn serialize_angle<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Serialize,
    for<'a> &'a T: ToString,
{
    SERIALIZE_WITH_UNITS.with(|ctx| {
        if ctx.get() {
            let mut string = value.to_string();
            string.push_str(" rad");
            string.serialize(serializer)
        } else {
            value.serialize(serializer)
        }
    })
}

/**
Like [`serialize_angle`], but serializes an [`&Option<T>`]
instead of a `&T` implementing [`Into<DynQuantity>`].

For examples see the [`serialize_with_units`] documentation.
 */
pub fn serialize_opt_angle<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Serialize,
    for<'a> &'a T: ToString,
{
    match value.as_ref() {
        Some(v) => serialize_angle(v, serializer),
        None => return serializer.serialize_none(),
    }
}

// =============================================================================

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
Like [`deserialize_quantity`], but deserializes into an [`Option<T>`]
instead of a `T` implementing [`TryFrom<DynQuantity>`].

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
    let quantity = Option::<InnerOrString<_>>::deserialize(deserializer)?;
    match quantity {
        Some(number_or_string) => {
            match number_or_string {
                InnerOrString::Inner(quantity) => Ok(Some(quantity)),
                #[cfg(feature = "from_str")]
                InnerOrString::String(string) => {
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
    let quantity = Option::<InnerOrString<f64>>::deserialize(deserializer)?;
    match quantity {
        Some(number_or_string) => {
            match number_or_string {
                InnerOrString::Inner(quantity) => Ok(Some(quantity)),
                #[cfg(feature = "from_str")]
                InnerOrString::String(string) => {
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
                #[cfg(feature = "from_str")]
                QuantityVecEnum::String(string) => {
                    fn parse_vec<T, E>(
                        input: &str,
                        multiplier: &DynQuantity<Complex<f64>>,
                    ) -> Result<Vec<T>, String>
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
                            let element: T =
                                quantity_mult.try_into().map_err(|err: E| err.to_string())?;
                            output.push(element)
                        }
                        return Ok(output);
                    }

                    // Remove the unit from the string by finding the closing
                    // bracket of the vector "]". The slice before the closing
                    // bracket can then be deserialized as a vector of floats,
                    // while the slice behind the closing bracket can be
                    // interpreted as Unit.
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
                                    &DynQuantity::new(Complex::new(1.0, 0.0), Unit::default()),
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

#[derive(DeserializeUntaggedVerboseError)]
enum QuantityVecEnum<T>
where
    T: TryFrom<DynQuantity<Complex<f64>>>,
    <T as TryFrom<DynQuantity<Complex<f64>>>>::Error: std::fmt::Display,
{
    Vec(Vec<T>),
    QuantityVec(QuantityVec<T>),
    #[cfg(feature = "from_str")]
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

                let first_value = match seq.next_element::<InnerOrString<T>>()? {
                    Some(element) => element,
                    None => return Ok(vec), // Empty vec
                };

                match first_value {
                    InnerOrString::Inner(number) => {
                        vec.0.push(number);
                        while let Some(quantity_rep) = seq.next_element::<InnerOrString<T>>()? {
                            match quantity_rep {
                                InnerOrString::Inner(quantity) => vec.0.push(quantity),
                                #[cfg(feature = "from_str")]
                                InnerOrString::String(_) => {
                                    return Err(serde::de::Error::custom(
                                        "either all elements of the vector must have the same quantity, or no element must have a quantity",
                                    ));
                                }
                            }
                        }
                    }
                    #[cfg(feature = "from_str")]
                    InnerOrString::String(string) => {
                        let first_element =
                            DynQuantity::from_str(&string).map_err(serde::de::Error::custom)?;
                        let output_element =
                            first_element.try_into().map_err(serde::de::Error::custom)?;
                        vec.0.push(output_element);

                        while let Some(quantity_rep) = seq.next_element::<InnerOrString<T>>()? {
                            // Loop through all other elements and check if their unit is equal to
                            // first_element_unit
                            match quantity_rep {
                                InnerOrString::Inner(_) => {
                                    return Err(serde::de::Error::custom(
                                        "either all elements of the vector must have the same quantity, or no element must have a quantity",
                                    ));
                                }
                                InnerOrString::String(string) => {
                                    let element = DynQuantity::<Complex<f64>>::from_str(&string)
                                        .map_err(serde::de::Error::custom)?;
                                    if element.unit != first_element.unit {
                                        return Err(serde::de::Error::custom(
                                            ConversionError::UnitMismatch {
                                                expected: first_element.unit,
                                                found: element.unit,
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
            let value: InnerOrString<DynQuantity<f64>> = serde_yaml::from_str("1.0").unwrap();
            match value {
                InnerOrString::Inner(value) => {
                    assert_eq!(value.value, 1.0);
                }
                InnerOrString::String(_) => unreachable!(),
            }
        }
        {
            let value: InnerOrString<DynQuantity<f64>> = serde_yaml::from_str("1.0 A").unwrap();
            match value {
                InnerOrString::Inner(value) => {
                    assert_eq!(value.value, 1.0);
                    assert_eq!(value.unit.ampere, 1);
                }
                InnerOrString::String(_) => unreachable!(),
            }
        }
        {
            let value: InnerOrString<DynQuantity<Complex<f64>>> =
                serde_yaml::from_str("1.0 A").unwrap();
            match value {
                InnerOrString::Inner(value) => {
                    assert_eq!(value.value.re, 1.0);
                    assert_eq!(value.value.im, 0.0);
                    assert_eq!(value.unit.ampere, 1);
                }
                InnerOrString::String(_) => unreachable!(),
            }
        }
    }
}
