/*!
This module contains the [`Unit`] struct and supporting code. See the
documentation string of [`Unit`] for more information.
*/

use std::ops::{Div, DivAssign, Mul, MulAssign};

use num::Complex;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::RootError;

#[cfg(feature = "serde")]
mod serde_impl;

/**
Struct representing a unit of measurement in the SI system via the exponents of
the base units. The unit is purely defined by the values of its fields, meaning
that it can change at runtime. The struct implements basic arithmetic functions
such as multiplication and division (via the [`Mul`], [`MulAssign`], [`Div`],
[`DivAssign`] traits), exponentiation ([`Unit::powi`]) and a fallible version
of root calculation ([`Unit::try_nthroot`]).

# Serialization and deserialization

If the **serde** feature is enabled, this struct can be serialized and
deserialized. Serialization creates the "standard"
[serde](https://crates.io/crates/serde) representation one would expect from the
`Serialize` macro.

An [`Unit`] can be deserialized from both its "standard" serialized
representation and from a [`PredefUnit`] variant:
```
use dyn_quantity::Unit;

// Direct deserialization
let str = "---\nsecond: -3\nmeter: 2\nkilogram: 1\nampere: -1\nkelvin: 0\nmol: 0\ncandela: 0";
let unit_direct: Unit = serde_yaml::from_str(str).unwrap();

// Deserialization from PredefUnit
let str = "ElectricVoltage";
let unit_predef: Unit = serde_yaml::from_str(str).unwrap();
assert_eq!(unit_predef, unit_direct);
```
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Unit {
    /// Exponent for the SI base unit of time.
    pub second: i32,
    /// Exponent for the SI base unit of length.
    pub meter: i32,
    /// Exponent for the SI base unit of mass.
    pub kilogram: i32,
    /// Exponent for the SI base unit of electrical current.
    pub ampere: i32,
    /// Exponent for the SI base unit of temperature.
    pub kelvin: i32,
    /// Exponent for the SI base unit of amount of substance.
    pub mol: i32,
    /// Exponent for the SI base unit of luminous intensity
    pub candela: i32,
}

impl From<[i32; 7]> for Unit {
    /**
    Converts an array of seven `i32` values into `Unit`.

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
        return Unit {
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

impl From<Unit> for [i32; 7] {
    /**
    Converts an `Unit` into an array of seven `i32`.

    The exponents are put into the array in the following order:
    - `array[0]`: Exponent of second
    - `array[1]`: Exponent of meter
    - `array[2]`: Exponent of kilogram
    - `array[3]`: Exponent of ampere
    - `array[4]`: Exponent of kelvin
    - `array[5]`: Exponent of mol
    - `array[6]`: Exponent of candela
     */
    fn from(value: Unit) -> Self {
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

impl Unit {
    /**
    Raises `self` to an integer power.

    # Examples
    ```
    use dyn_quantity::Unit;

    let exponents = Unit::from([0, 1, 0, 2, 0, -2, 0]);
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
    use dyn_quantity::Unit;

    let unit = Unit::from([0, 2, 0, 2, 0, -4, 0]);

    // It is possible to calculate the square root:
    let array: [i32; 7] = unit.clone().try_nthroot(2).unwrap().into();
    assert_eq!(array, [0, 1, 0, 1, 0, -2, 0]);

    // But not the cubic root (not all exponents are divisible by 3):
    assert!(unit.try_nthroot(3).is_err());
    ```
     */
    pub fn try_nthroot(mut self, n: i32) -> Result<Self, RootError> {
        fn try_nthroot_inner(unit: &Unit, exp: i32, n: i32) -> Result<i32, RootError> {
            if exp % n == 0 {
                return Ok(exp / n);
            } else {
                return Err(RootError {
                    n,
                    unit: unit.clone(),
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

    /// Returns whether [`Unit`] is dimensionless (all exponents are zero) or not.
    pub fn is_dimensionless(&self) -> bool {
        return self.second == 0
            && self.meter == 0
            && self.kilogram == 0
            && self.ampere == 0
            && self.kelvin == 0
            && self.mol == 0
            && self.candela == 0;
    }
}

impl std::fmt::Display for Unit {
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

impl Mul for Unit {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.mul_assign(rhs);
        return self;
    }
}

impl MulAssign for Unit {
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

impl Div for Unit {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.div_assign(rhs);
        return self;
    }
}

impl DivAssign for Unit {
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
A trait to derive [`Unit`] from a type. This trait bridges the gap
between (external) types representing physical quantities (such as e.g. the
[`Quantity`](https://docs.rs/uom/latest/uom/si/struct.Quantity.html) type from
the [uom](https://crates.io/crates/uom) crate) and [`Unit`].
 */
pub trait UnitFromType {
    /**
    This function derives an [`Unit`] from any type which implements
    [`UnitFromType`]. Its default implementation returns an [`Unit`]
    where all exponents are zero.

    # Examples
    ```
    use dyn_quantity::UnitFromType;
    use uom::si::f64::Length;

    // 64-bit floats do not represent a physical quantity
    let exp = f64::unit_from_type();
    assert_eq!(exp.meter, 0);

    // The "Length" type alias from the uom crate represents a physical quantity (length)
    let exp = Length::unit_from_type();
    assert_eq!(exp.meter, 1);
    ```
    */
    fn unit_from_type() -> Unit {
        return Unit::default();
    }
}

impl UnitFromType for f64 {}

impl UnitFromType for Complex<f64> {}

/**
An enum representing predefined [`Unit`]s.

This enum serves two purposes:
* It can be a constructor for [`Unit`] via the [`From`] / [`Into`] implementations:
```
use dyn_quantity::{Unit, PredefUnit};

let unit: Unit = PredefUnit::ElectricCurrent.into();
assert_eq!(unit.ampere, 1);

let unit: Unit = PredefUnit::Area.into();
assert_eq!(unit.meter, 2);
```
* When deserializing an [`Unit`], it can be used instead of the explicit struct
implementation:
```
use dyn_quantity::Unit;

let str = "Volume";
let unit: Unit = serde_yaml::from_str(str).unwrap();
assert_eq!(unit.meter, 3);
```
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PredefUnit {
    /// SI base units representation: s (second)
    Time,
    /// SI base units representation: m (meter)
    Length,
    /// SI base units representation: kg (kilogram)
    Mass,
    /// SI base units representation: A (ampere)
    ElectricCurrent,
    /// SI base units representation: K (kelvin)
    Temperature,
    /// SI base units representation: mol (mol)
    AmountOfSubstance,
    /// SI base units representation: cd (candela)
    LuminousIntensity,
    /// SI base units representation: m^2 (square meter)
    Area,
    /// SI base units representation: m^3 (cubic meter)
    Volume,
    /// SI base units representation: s^-3*m^2*kg*A*-1 (voltage)
    ElectricVoltage,
    /// SI base units representation: s^-2*m*kg (newton)
    Force,
    /// SI base units representation: s^-2*m^2*kg (newton meter)
    Torque,
    /// SI base units representation: s^-3*m^2*kg (watt)
    Power,
    /// SI base units representation: s^-2*m^2*kg (energy)
    Energy,
    /// SI base units representation: s^-1 (hertz)
    Frequency,
    /// SI base units representation: s^-1*m (meter per second)
    Velocity,
    /// SI base units representation: s^-1 (rad per second)
    AngularVelocity,
    /// SI base units representation: s^-2*m^2*kg*A^-1 (weber)
    MagneticFlux,
    /// SI base units representation: s^-2*kg*A^-1 (tesla)
    MagneticFluxDensity,
    /// SI base units representation: m*A^-1 (ampere per meter)
    MagneticFieldStrength,
    /// SI base units representation: s^-2*m^2*kg*A^-2 (henry)
    Inductance,
    /// SI base units representation: s^3*m^-2*kg^-1*A^2 (siemens)
    ElectricConductance,
    /// SI base units representation: s^-3*m^2*kg*A^-2 (ohm)
    ElectricResistance,
    /// SI base units representation: s^3*m^-2*kg^-1*A^2 (siemens)
    ElectricConductivity,
    /// SI base units representation: s^-3*m^3*kg*A^-2 (ohm)
    ElectricResistivity,
}

impl From<PredefUnit> for Unit {
    fn from(value: PredefUnit) -> Self {
        match value {
            PredefUnit::Time => Self {
                second: 1,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Length => Self {
                second: 0,
                meter: 1,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Mass => Self {
                second: 0,
                meter: 0,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::ElectricCurrent => Self {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Temperature => Self {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 1,
                mol: 0,
                candela: 0,
            },
            PredefUnit::AmountOfSubstance => Self {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 1,
                candela: 0,
            },
            PredefUnit::LuminousIntensity => Self {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 1,
            },
            PredefUnit::Area => Self {
                second: 0,
                meter: 2,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Volume => Self {
                second: 0,
                meter: 3,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::ElectricVoltage => Self {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Force => Self {
                second: -2,
                meter: 1,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Torque => Self {
                second: -2,
                meter: 2,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Power => Self {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Energy => Self {
                second: -2,
                meter: 2,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Frequency => Self {
                second: -1,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Velocity => Self {
                second: -1,
                meter: 1,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::AngularVelocity => Self {
                second: -1,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::MagneticFlux => Self {
                second: -2,
                meter: 2,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::MagneticFluxDensity => Self {
                second: -2,
                meter: 0,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::MagneticFieldStrength => Self {
                second: 0,
                meter: -1,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::Inductance => Self {
                second: -2,
                meter: 2,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::ElectricConductance => Self {
                second: 3,
                meter: -2,
                kilogram: -1,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::ElectricResistance => Self {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::ElectricConductivity => Self {
                second: 3,
                meter: -3,
                kilogram: -1,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
            PredefUnit::ElectricResistivity => Self {
                second: -3,
                meter: 3,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0,
            },
        }
    }
}
