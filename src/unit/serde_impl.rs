/*!
This module is only available if the [`serde`] feature is enabled.
It contains the serialization and deserialization implementations for
[`Unit`].
*/

use super::{CommonUnits, Unit};
use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Unit", 7)?;
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

#[derive(serde::Deserialize)]
struct UnitAlias {
    second: i32,
    meter: i32,
    kilogram: i32,
    ampere: i32,
    kelvin: i32,
    mol: i32,
    candela: i32,
}

/**
An [`Unit`] can be deserialized a couple of different representations.
 */
#[derive(DeserializeUntaggedVerboseError)]
enum UnitVariants {
    /**
    Native representation of [`Unit`] (via an alias struct in order
    to avoid infinite recursion)-
     */
    Unit(UnitAlias),
    /**
    Deserialization from a [`CommonUnits`].
     */
    CommonUnits(CommonUnits),
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Unit, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variants = UnitVariants::deserialize(deserializer)?;
        match variants {
            UnitVariants::Unit(alias) => {
                return Ok(Unit {
                    second: alias.second,
                    meter: alias.meter,
                    kilogram: alias.kilogram,
                    ampere: alias.ampere,
                    kelvin: alias.kelvin,
                    mol: alias.mol,
                    candela: alias.candela,
                });
            }
            UnitVariants::CommonUnits(common_units) => {
                return Ok(common_units.into());
            }
        }
    }
}
