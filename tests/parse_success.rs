use std::f64::{INFINITY, NEG_INFINITY, consts::PI};

use dyn_quantity::*;
use num::Complex;
use std::str::FromStr;

#[test]
fn test_addition() {
    {
        let quantity = DynQuantity::<f64>::from_str("3 A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 3.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("+3 A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 3.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-3 A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, -3.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("3 A + 1 A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 4.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("3 A + 1 kA").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1003.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }

    // Mismatching types (trying to add ampere to seconds)
    {
        let error = DynQuantity::<f64>::from_str("3 A + 1 s").unwrap_err();
        match error.reason {
            ParseErrorReason::UnitsOfSummandsNotIdentical(_) => (),
            _ => panic!("wrong error type"),
        }
    }
}

#[test]
fn test_addition_with_brackets() {
    // Add with brackets
    {
        let quantity = DynQuantity::<f64>::from_str("(3 A + 1 kA)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1003.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 A + (3 A + 1 kA)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1004.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 A + (-3 A + 1 kA)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 998.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("12 A - (3 A + 5 A)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 4.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("12 A - (-3 A + 5 A)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 10.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }

    // Multiply and divide
    {
        let quantity = DynQuantity::<f64>::from_str("1 A * 2A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 2.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 A * (2+8)A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 100.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 A * A(2+8)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 100.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 (2+8) A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 100.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_division() {
    {
        let quantity = DynQuantity::<f64>::from_str("2 / 2 A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("(2 / 2 A)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("(2 / 2) A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("12 A / (1 + 5) * 2A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 4.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("12 A / (1 + 5) 2A").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 4.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("12 A / ((1 + 5 )2A)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("12 A / (2A + 10 A)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 A / (2A)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 0.5, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 A / 2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 0.5, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("4 / 2 / 2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("4 / (2 / 2)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 4.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-4 / -4").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-4 / -4 + 1").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 2.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-4 / (2 + 2) + 1").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 0.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_parse_power_bracket() {
    {
        let quantity = DynQuantity::<f64>::from_str("(2A+8A)^2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 100.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 / (2A+8A)^2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 0.01, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("(2*s^3)^2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 4.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 6,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("8 s / (2*s^3)^2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 2.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -5,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("8 s / ((2*s^3)^2)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 2.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -5,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("8 s / ((2*s^3))^2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 2.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -5,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("4e2 mWb / (2*s^3)^2").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 0.1, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -8,
                meter: 2,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_parse_nested_brackets() {
    {
        let quantity = DynQuantity::<f64>::from_str("(((2A)) + 1 A)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 3.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_parse_real_quantities() {
    {
        let quantity = DynQuantity::<f64>::from_str("2.0 A/m * 3.0 H/m").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 6.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 0,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("2/PI N").unwrap();
        assert_eq!(quantity.value, 2.0 / PI);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 1,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("2/(PI N)").unwrap();
        assert_eq!(quantity.value, 2.0 / PI);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 2,
                meter: -1,
                kilogram: -1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/56 MS*m^-1").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1e6 / 56.0, epsilon = 1e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 3,
                meter: -3,
                kilogram: -1,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/56 MS/mm").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1e9 / 56.0, epsilon = 1e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 3,
                meter: -3,
                kilogram: -1,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/56 MS*mm^-1").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 1e9 / 56.0, epsilon = 1e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 3,
                meter: -3,
                kilogram: -1,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("4.0 V / (2.0 mA)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 2e3, epsilon = 1e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("4.0 V / 2.0 / mA").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 2e3, epsilon = 1e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/4 s^3/s^2").unwrap();
        assert_eq!(quantity.value, 0.25);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 1,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/4 Ohm*mm").unwrap();
        assert_eq!(quantity.value, 0.25e-3);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 3,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/4 ohm*mm").unwrap();
        assert_eq!(quantity.value, 0.25e-3);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 3,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 V").unwrap();
        assert_eq!(quantity.value, 10.0);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("PI µV/mA").unwrap();
        assert_eq!(quantity.value, PI * 1e-3);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/4 mm^2").unwrap();
        assert_eq!(quantity.value, 0.25e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 2,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/4 mm*mm").unwrap();
        assert_eq!(quantity.value, 0.25e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 2,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("300 kW").unwrap();
        assert_eq!(quantity.value, 3e5);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("300 GJ/W").unwrap();
        assert_eq!(quantity.value, 3e11);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 1,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("3 t").unwrap();
        assert_eq!(quantity.value, 3e3);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("3 mW / kg").unwrap();
        assert_eq!(quantity.value, 3e-3);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 2,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("210 mH").unwrap();
        assert_eq!(quantity.value, 0.21);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 2,
                kilogram: 1,
                ampere: -2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("10 A m").unwrap();
        assert_eq!(quantity.value, 10.0);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 1,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_tesla() {
    {
        let quantity = DynQuantity::<f64>::from_str("2.0 A/m * 3.0 H/m").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 6.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 0,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("(2.0 A/m * 3.0 H/m) + 0.5 T").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 6.5, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 0,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("2.0 A/m * 3.0 H/m + 0.5 T").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 6.5, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 0,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("0.5 T + 2.0 A/m * 3.0 H/m").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 6.5, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 0,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("0.5 T + (2.0 A/m * 3.0 H/m)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 6.5, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -2,
                meter: 0,
                kilogram: 1,
                ampere: -1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_percentage() {
    {
        let quantity = DynQuantity::<f64>::from_str("0.4 %").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 0.004, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1.0 / (1.0 %)").unwrap();
        approx::assert_abs_diff_eq!(quantity.value, 100.0, epsilon = 1e-8);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("0.4 % / K").unwrap();
        assert_eq!(quantity.value, 0.004);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: -1,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_parse_power_of_ten() {
    {
        let quantity = DynQuantity::<f64>::from_str("4 pi 1e-7 s").unwrap();
        assert_eq!(quantity.value, 4.0 * PI * 1e-7);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 1,
                meter: 0,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("3e6 W").unwrap();
        assert_eq!(quantity.value, 3.0 * 1e6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: -3,
                meter: 2,
                kilogram: 1,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("2.7e6 S/m").unwrap();
        assert_eq!(quantity.value, 2.7e6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 3,
                meter: -3,
                kilogram: -1,
                ampere: 2,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1/(2.0e6) m").unwrap();
        assert_eq!(quantity.value, 0.5e-6);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 1,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_parse_angle() {
    let quantity = DynQuantity::<f64>::from_str("180 degree/s").unwrap();
    assert_eq!(quantity.value, PI);
    assert_eq!(
        quantity.exponents,
        UnitExponents {
            second: -1,
            meter: 0,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0
        }
    )
}

#[test]
fn test_parse_complex() {
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("(1 + 2i) A").unwrap();
        assert_eq!(quantity.value.re, 1.0);
        assert_eq!(quantity.value.im, 2.0);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("(1 + 2i) / 1i A").unwrap();
        assert_eq!(quantity.value.re, 2.0);
        assert_eq!(quantity.value.im, -1.0);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("pi*1j A").unwrap();
        assert_eq!(quantity.value.re, 0.0);
        assert_eq!(quantity.value.im, PI);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("pi i A").unwrap();
        assert_eq!(quantity.value.re, 0.0);
        assert_eq!(quantity.value.im, PI);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("pi j A").unwrap();
        assert_eq!(quantity.value.re, 0.0);
        assert_eq!(quantity.value.im, PI);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        )
    }
}

#[test]
fn test_parse_no_value() {
    {
        let quantity = DynQuantity::<f64>::from_str("A*s").unwrap();
        assert_eq!(quantity.value, 1.0);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 1,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
    {
        let quantity = DynQuantity::<f64>::from_str(" A*s").unwrap();
        assert_eq!(quantity.value, 1.0);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 1,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
}

#[test]
fn test_parse_infinite() {
    {
        let quantity = DynQuantity::<f64>::from_str("inf A").unwrap();
        assert_eq!(quantity.value, INFINITY);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
    {
        let quantity = DynQuantity::<f64>::from_str(".inf A").unwrap();
        assert_eq!(quantity.value, INFINITY);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-inf A").unwrap();
        assert_eq!(quantity.value, NEG_INFINITY);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-.inf A").unwrap();
        assert_eq!(quantity.value, NEG_INFINITY);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("inf j A").unwrap();
        assert_eq!(quantity.value.re, 0.0);
        assert_eq!(quantity.value.im, INFINITY);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: 0,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-.inf / m^2").unwrap();
        assert_eq!(quantity.value, NEG_INFINITY);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: -2,
                kilogram: 0,
                ampere: 0,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
    {
        let quantity = DynQuantity::<f64>::from_str("-.inf A / m").unwrap();
        assert_eq!(quantity.value, NEG_INFINITY);
        assert_eq!(
            quantity.exponents,
            UnitExponents {
                second: 0,
                meter: -1,
                kilogram: 0,
                ampere: 1,
                kelvin: 0,
                mol: 0,
                candela: 0
            }
        );
    }
}
