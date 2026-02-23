use std::{f64::consts::PI, str::FromStr};

use dyn_quantity::uom::si::{
    complex64, electrical_resistance::ohm, energy::joule, f64::*, length::meter,
    specific_power::watt_per_kilogram, torque::newton_millimeter,
};
use dyn_quantity::*;
use num::Complex;

#[test]
fn test_conversion_real() {
    {
        let quantity = DynQuantity::<f64>::from_str("1 km").unwrap();
        let quantity_uom: Length = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.get::<meter>(), 1000.0);
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 kV / 2 / A").unwrap();
        let quantity_uom: ElectricalResistance = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.get::<ohm>(), 500.0);
    }
    {
        let quantity = DynQuantity::<f64>::from_str("pi W * kg^-1").unwrap();
        let quantity_uom: SpecificPower = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.get::<watt_per_kilogram>(), PI);
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 N * m").unwrap();
        let quantity_uom: Energy = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.get::<joule>(), 10.0);
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 N * m").unwrap();
        let quantity_uom: Torque = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.get::<newton_millimeter>(), 10000.0);
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 Nm").unwrap();
        let quantity_uom: Energy = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.get::<joule>(), 10.0);
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 Nm").unwrap();
        let quantity_uom: Torque = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.get::<newton_millimeter>(), 10000.0);
    }
}

#[test]
fn test_conversion_complex() {
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("(1 + 2j) A").unwrap();
        let quantity_uom: complex64::ElectricCurrent = quantity.try_into().unwrap();
        assert_eq!(quantity_uom.value, Complex::new(1.0, 2.0));
    }
}

#[test]
fn test_unit_exponents_from_uom() {
    {
        let exponents = Power::unit_from_type();
        assert_eq!(exponents.second, -3);
        assert_eq!(exponents.meter, 2);
        assert_eq!(exponents.kilogram, 1);
        assert_eq!(exponents.ampere, 0);
        assert_eq!(exponents.kelvin, 0);
        assert_eq!(exponents.candela, 0);
        assert_eq!(exponents.mol, 0);
    }
    {
        let exponents = ElectricCurrent::unit_from_type();
        assert_eq!(exponents.second, 0);
        assert_eq!(exponents.meter, 0);
        assert_eq!(exponents.kilogram, 0);
        assert_eq!(exponents.ampere, 1);
        assert_eq!(exponents.kelvin, 0);
        assert_eq!(exponents.candela, 0);
        assert_eq!(exponents.mol, 0);
    }
}
