use std::str::FromStr;

use dyn_quantity::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Quantities {
    #[serde(
        serialize_with = "serialize_quantity",
        deserialize_with = "deserialize_quantity"
    )]
    quantity: DynQuantity<f64>,
    #[serde(
        serialize_with = "serialize_opt_quantity",
        deserialize_with = "deserialize_opt_quantity"
    )]
    opt_quantity: Option<DynQuantity<f64>>,
}

#[test]
fn test_serialize() {
    let quantities = Quantities {
        quantity: DynQuantity::from_str("0.001 ohm").expect("valid string"),
        opt_quantity: Some(DynQuantity::from_str("1000 V").expect("valid string")),
    };

    // Without units (standard serialization)
    let expected = indoc::indoc! {"
        ---
        quantity:
          value: 0.001
          unit:
            second: -3
            meter: 2
            kilogram: 1
            ampere: -2
            kelvin: 0
            mol: 0
            candela: 0
        opt_quantity:
          value: 1000.0
          unit:
            second: -3
            meter: 2
            kilogram: 1
            ampere: -1
            kelvin: 0
            mol: 0
            candela: 0
        "};
    let actual = serde_yaml::to_string(&quantities).expect("serialization succeeds");
    assert_eq!(expected, actual);

    // With units
    let expected = indoc::indoc! {"
        ---
        quantity: 0.001 s^-3 m^2 kg A^-2
        opt_quantity: 1000 s^-3 m^2 kg A^-1
        "};
    let actual = serialize_with_units(|| serde_yaml::to_string(&quantities))
        .expect("serialization succeeds");
    assert_eq!(expected, actual);
}
