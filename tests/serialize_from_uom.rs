use dyn_quantity::*;
use serde::{Deserialize, Serialize};
use uom::si::{
    electric_potential::millivolt,
    f64::*,
    length::{kilometer, millimeter},
    magnetic_flux_density::tesla,
    mass_density::kilogram_per_cubic_meter,
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
        // None
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

#[test]
fn test_complicated_units() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct ComposedQuantities {
        #[serde(
            serialize_with = "serialize_quantity",
            deserialize_with = "deserialize_quantity"
        )]
        magnetic_flux_density: MagneticFluxDensity,
        #[serde(
            serialize_with = "serialize_quantity",
            deserialize_with = "deserialize_quantity"
        )]
        voltage: ElectricPotential,
        #[serde(
            serialize_with = "serialize_quantity",
            deserialize_with = "deserialize_quantity"
        )]
        mass_density: MassDensity,
    }
    let ser_composed = ComposedQuantities {
        magnetic_flux_density: MagneticFluxDensity::new::<tesla>(0.5),
        voltage: ElectricPotential::new::<millivolt>(2.0),
        mass_density: MassDensity::new::<kilogram_per_cubic_meter>(8000.0),
    };
    let expected = indoc::indoc! {"
    ---
    magnetic_flux_density: 0.5 s^-2 kg A^-1
    voltage: 0.002 s^-3 m^2 kg A^-1
    mass_density: 8000 m^-3 kg
    "};
    let actual = serialize_with_units(|| serde_yaml::to_string(&ser_composed))
        .expect("serialization succeeds");
    assert_eq!(expected, actual);

    let de_composed = serde_yaml::from_str(&actual).expect("deserialization succeeds");
    assert_eq!(ser_composed, de_composed);
}
