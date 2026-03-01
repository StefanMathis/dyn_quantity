use dyn_quantity::*;
use serde::{Deserialize, Serialize};
use uom::si::{
    f64::Length,
    length::{kilometer, millimeter},
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Quantities {
    #[serde(
        serialize_with = "serialize_quantity",
        deserialize_with = "deserialize_quantity"
    )]
    length: Length,
    #[serde(
        serialize_with = "serialize_opt_quantity",
        deserialize_with = "deserialize_opt_quantity"
    )]
    opt_length: Option<Length>,
    #[serde(
        serialize_with = "serialize_angle",
        deserialize_with = "deserialize_angle"
    )]
    angle: f64,
    #[serde(
        serialize_with = "serialize_opt_angle",
        deserialize_with = "deserialize_opt_angle"
    )]
    opt_angle: Option<f64>,
}

#[test]
fn test_serialize_with_units() {
    {
        // Some
        let quantities = Quantities {
            length: Length::new::<millimeter>(1.0),
            opt_length: Some(Length::new::<kilometer>(1.0)),
            angle: 1.0,
            opt_angle: Some(2.0),
        };

        // Without units (standard serialization)

        let expected = indoc::indoc! {"
        ---
        length: 0.001
        opt_length: 1000.0
        angle: 1.0
        opt_angle: 2.0
        "};
        let actual = serde_yaml::to_string(&quantities).expect("serialization succeeds");
        assert_eq!(expected, actual);

        // With units
        let expected = indoc::indoc! {"
        ---
        length: 0.001 m
        opt_length: 1000 m
        angle: 1 rad
        opt_angle: 2 rad
        "};
        let actual = serialize_with_units(|| serde_yaml::to_string(&quantities))
            .expect("serialization succeeds");
        assert_eq!(expected, actual);
    }
    {
        // Nine
        let quantities = Quantities {
            length: Length::new::<millimeter>(1.0),
            opt_length: None,
            angle: 1.0,
            opt_angle: None,
        };

        // Without units (standard serialization)

        let expected = indoc::indoc! {"
        ---
        length: 0.001
        opt_length: ~
        angle: 1.0
        opt_angle: ~
        "};
        let actual = serde_yaml::to_string(&quantities).expect("serialization succeeds");
        assert_eq!(expected, actual);

        // With units
        let expected = indoc::indoc! {"
        ---
        length: 0.001 m
        opt_length: ~
        angle: 1 rad
        opt_angle: ~
        "};
        let actual = serialize_with_units(|| serde_yaml::to_string(&quantities))
            .expect("serialization succeeds");
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_serialize_and_deserialize() {
    {
        // Some
        let ser_quantities = Quantities {
            length: Length::new::<millimeter>(1.0),
            opt_length: Some(Length::new::<kilometer>(1.0)),
            angle: 1.0,
            opt_angle: Some(2.0),
        };
        let ser = serialize_with_units(|| serde_yaml::to_string(&ser_quantities))
            .expect("serialization succeeds");
        let de_quantities = serde_yaml::from_str(&ser).expect("deserialization succeeds");
        assert_eq!(ser_quantities, de_quantities);
    }
    {
        // None
        let ser_quantities = Quantities {
            length: Length::new::<millimeter>(1.0),
            opt_length: None,
            angle: 1.0,
            opt_angle: None,
        };
        let ser = serialize_with_units(|| serde_yaml::to_string(&ser_quantities))
            .expect("serialization succeeds");
        let de_quantities = serde_yaml::from_str(&ser).expect("deserialization succeeds");
        assert_eq!(ser_quantities, de_quantities);
    }
}
