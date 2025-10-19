/*!
This module is only available if the [`serde`] feature is enabled.
It contains the serialization and deserialization implementations for
[`DynQuantity`] and [`UnitExponents`].
*/

use crate::{DynQuantity, F64RealOrComplex, UnitExponents};
use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[cfg(feature = "from_str")]
use std::str::FromStr;

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
        state.serialize_field("exponents", &self.exponents)?;
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
pub(crate) enum QuantityVariants<V>
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
pub(super) struct QuantityAlias<V: F64RealOrComplex> {
    value: V,
    exponents: UnitExponents,
}

impl<V: F64RealOrComplex> TryFrom<QuantityVariants<V>> for DynQuantity<V> {
    type Error = super::ParseError;

    fn try_from(variant: QuantityVariants<V>) -> Result<Self, Self::Error> {
        match variant {
            QuantityVariants::Quantity(variant) => {
                return Ok(Self {
                    value: variant.value,
                    exponents: variant.exponents,
                });
            }
            #[cfg(feature = "from_str")]
            QuantityVariants::String(string) => {
                return Self::from_str(&string);
            }
            QuantityVariants::Value(value) => {
                return Ok(Self {
                    value,
                    exponents: UnitExponents::default(),
                });
            }
        }
    }
}

impl Serialize for UnitExponents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UnitExponents", 7)?;
        state.serialize_field("second", &self.second)?;
        state.serialize_field("meter", &self.meter)?;
        state.serialize_field("kilogram", &self.kilogram)?;
        state.serialize_field("ampere", &self.ampere)?;
        state.serialize_field("kelvin", &self.kelvin)?;
        state.serialize_field("mol", &self.mol)?;
        state.serialize_field("candela", &self.candela)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for UnitExponents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Second,
            Meter,
            Kilogram,
            Ampere,
            Kelvin,
            Mol,
            Candela,
        }

        struct UnitExponentsVisitor;

        impl<'de> Visitor<'de> for UnitExponentsVisitor {
            type Value = UnitExponents;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct UnitExponents")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<UnitExponents, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let second = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let meter = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let kilogram = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let ampere = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let kelvin = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let mol = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let candela = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(UnitExponents {
                    second,
                    meter,
                    kilogram,
                    ampere,
                    kelvin,
                    mol,
                    candela,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<UnitExponents, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut second = None;
                let mut meter = None;
                let mut kilogram = None;
                let mut ampere = None;
                let mut kelvin = None;
                let mut mol = None;
                let mut candela = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Second => {
                            if second.is_some() {
                                return Err(serde::de::Error::duplicate_field("second"));
                            }
                            second = Some(map.next_value()?);
                        }
                        Field::Meter => {
                            if meter.is_some() {
                                return Err(serde::de::Error::duplicate_field("meter"));
                            }
                            meter = Some(map.next_value()?);
                        }
                        Field::Kilogram => {
                            if kilogram.is_some() {
                                return Err(serde::de::Error::duplicate_field("kilogram"));
                            }
                            kilogram = Some(map.next_value()?);
                        }
                        Field::Ampere => {
                            if ampere.is_some() {
                                return Err(serde::de::Error::duplicate_field("ampere"));
                            }
                            ampere = Some(map.next_value()?);
                        }
                        Field::Kelvin => {
                            if kelvin.is_some() {
                                return Err(serde::de::Error::duplicate_field("kelvin"));
                            }
                            kelvin = Some(map.next_value()?);
                        }
                        Field::Mol => {
                            if mol.is_some() {
                                return Err(serde::de::Error::duplicate_field("mol"));
                            }
                            mol = Some(map.next_value()?);
                        }
                        Field::Candela => {
                            if candela.is_some() {
                                return Err(serde::de::Error::duplicate_field("candela"));
                            }
                            candela = Some(map.next_value()?);
                        }
                    }
                }

                let second = second.ok_or_else(|| serde::de::Error::missing_field("second"))?;
                let meter = meter.ok_or_else(|| serde::de::Error::missing_field("meter"))?;
                let kilogram =
                    kilogram.ok_or_else(|| serde::de::Error::missing_field("kilogram"))?;
                let ampere = ampere.ok_or_else(|| serde::de::Error::missing_field("ampere"))?;
                let kelvin = kelvin.ok_or_else(|| serde::de::Error::missing_field("kelvin"))?;
                let mol = mol.ok_or_else(|| serde::de::Error::missing_field("mol"))?;
                let candela = candela.ok_or_else(|| serde::de::Error::missing_field("candela"))?;

                Ok(UnitExponents {
                    second,
                    meter,
                    kilogram,
                    ampere,
                    kelvin,
                    mol,
                    candela,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "second", "meter", "kilogram", "ampere", "kelvin", "mol", "candela",
        ];
        deserializer.deserialize_struct("UnitExponents", FIELDS, UnitExponentsVisitor)
    }
}
