use std::f64::consts::{FRAC_PI_2, PI, TAU};

use dyn_quantity::uom::si::{electrical_resistance::ohm, f64::*, torque::newton_meter};
use dyn_quantity::*;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use uom::si::magnetic_flux_density::tesla;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct LengthWrapper {
    #[serde(deserialize_with = "deserialize_quantity")]
    quantity: Length,
}

#[test]
fn test_deserialize_length() {
    let first_val = indoc! {"
        ---
        quantity: 1000 mm
        "};
    let first_val_de: LengthWrapper = serde_yaml::from_str(first_val).unwrap();

    let second_val = indoc! {"
        ---
        quantity: 1 m
        "};
    let second_val_de: LengthWrapper = serde_yaml::from_str(second_val).unwrap();

    let third_val = indoc! {"
        ---
        quantity: 1
        "};
    let third_val_de: LengthWrapper = serde_yaml::from_str(third_val).unwrap();

    assert_eq!(first_val_de, second_val_de);
    assert_eq!(first_val_de, third_val_de);
    assert_eq!(second_val_de, third_val_de);

    let string = serde_yaml::to_string(&first_val_de).unwrap();
    let fourth_val_de: LengthWrapper = serde_yaml::from_str(&string).unwrap();

    assert_eq!(first_val_de, fourth_val_de);
}

#[test]
fn test_dimension_mismatch() {
    use indoc::indoc;
    let second_val = indoc! {"
        ---
        quantity: 1 kg
        "};
    let err: serde_yaml::Error = serde_yaml::from_str::<LengthWrapper>(second_val).unwrap_err();
    assert_eq!(
        "expected s^0 m^1 kg^0 A^0 K^0 mol^0 cd^0, found s^0 m^0 kg^1 A^0 K^0 mol^0 cd^0 at line 2 column 9",
        err.to_string()
    );
}

#[test]
fn test_deserialize_from_quantity_vec_real() {
    {
        #[derive(Deserialize, PartialEq, Debug)]
        struct VecWrapper {
            #[serde(deserialize_with = "deserialize_vec_of_quantities")]
            vec: Vec<MagneticFluxDensity>,
        }

        let no_units = indoc! {"
            ---
            vec: [2.0, 3.0, 4.0]
            "};
        let no_units_de: VecWrapper = serde_yaml::from_str(no_units).unwrap();
        assert_eq!(no_units_de.vec[0].get::<tesla>(), 2.0);
        assert_eq!(no_units_de.vec[1].get::<tesla>(), 3.0);
        assert_eq!(no_units_de.vec[2].get::<tesla>(), 4.0);

        let direct_deserialization = indoc! {"
            ---
            vec: [2.0 T, 3.0 T, 4.0 T]
            "};
        let direct_de: VecWrapper = serde_yaml::from_str(direct_deserialization).unwrap();
        assert_eq!(direct_de.vec[0].get::<tesla>(), 2.0);
        assert_eq!(direct_de.vec[1].get::<tesla>(), 3.0);
        assert_eq!(direct_de.vec[2].get::<tesla>(), 4.0);

        let indirect_deserialization_1 = indoc! {"
            ---
            vec: '[2.0, 3.0, 4.0] T'
            "};
        let indirect_de_1: VecWrapper = serde_yaml::from_str(indirect_deserialization_1).unwrap();
        assert_eq!(indirect_de_1.vec[0].get::<tesla>(), 2.0);
        assert_eq!(indirect_de_1.vec[1].get::<tesla>(), 3.0);
        assert_eq!(indirect_de_1.vec[2].get::<tesla>(), 4.0);

        let indirect_deserialization_2 = indoc! {"
            ---
            vec: '[2000.0, 3000.0, 4000.0] mT'
            "};
        let indirect_de_2: VecWrapper = serde_yaml::from_str(indirect_deserialization_2).unwrap();

        // Slight rounding errors may occur due to the conversion from mT to T
        approx::assert_abs_diff_eq!(indirect_de_2.vec[0].get::<tesla>(), 2.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(indirect_de_2.vec[1].get::<tesla>(), 3.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(indirect_de_2.vec[2].get::<tesla>(), 4.0, epsilon = 1e-15);
    }

    {
        #[derive(Deserialize, PartialEq, Debug)]
        struct VecWrapper {
            #[serde(deserialize_with = "deserialize_vec_of_quantities")]
            vec: Vec<MagneticFluxDensity>,
        }

        // Deserialization fails since we expect a vector of real quantities
        let direct_deserialization = indoc! {"
        ---
        vec: [2.0 T, (3.0 + 1i) T, 4.0 T]
        "};
        assert!(serde_yaml::from_str::<VecWrapper>(direct_deserialization).is_err());
    }

    {
        // Combined unit
        #[derive(Deserialize, PartialEq, Debug)]
        struct VecWrapper {
            #[serde(deserialize_with = "deserialize_vec_of_quantities")]
            vec: Vec<ElectricalResistance>,
        }

        let str = indoc! {"
        ---
        vec: [2000.0 mOhm, 3000.0 mOhm, 4000.0 mOhm]
        "};
        let var1: VecWrapper = serde_yaml::from_str(str).unwrap();
        approx::assert_abs_diff_eq!(var1.vec[0].get::<ohm>(), 2.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var1.vec[1].get::<ohm>(), 3.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var1.vec[2].get::<ohm>(), 4.0, epsilon = 1e-15);

        let str = indoc! {"
        ---
        vec: [2 V/A, 3 V/A, 4000.0 mV/A]
        "};
        let var2: VecWrapper = serde_yaml::from_str(str).unwrap();
        approx::assert_abs_diff_eq!(var2.vec[0].get::<ohm>(), 2.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var2.vec[1].get::<ohm>(), 3.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var2.vec[2].get::<ohm>(), 4.0, epsilon = 1e-15);

        let str = indoc! {"
        ---
        vec: '[2000, 3000, 4000] mV/A'
        "};
        let var3: VecWrapper = serde_yaml::from_str(str).unwrap();
        approx::assert_abs_diff_eq!(var3.vec[0].get::<ohm>(), 2.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var3.vec[1].get::<ohm>(), 3.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var3.vec[2].get::<ohm>(), 4.0, epsilon = 1e-15);

        let str = indoc! {"
        ---
        vec: '[2 V, 3 V, 4 V] 1/A'
        "};
        let var4: VecWrapper = serde_yaml::from_str(str).unwrap();
        approx::assert_abs_diff_eq!(var4.vec[0].get::<ohm>(), 2.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var4.vec[1].get::<ohm>(), 3.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var4.vec[2].get::<ohm>(), 4.0, epsilon = 1e-15);

        let str = indoc! {"
        ---
        vec: '[2 / A, 3 / A, 4 / A] V'
        "};
        let var5: VecWrapper = serde_yaml::from_str(str).unwrap();
        approx::assert_abs_diff_eq!(var5.vec[0].get::<ohm>(), 2.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var5.vec[1].get::<ohm>(), 3.0, epsilon = 1e-15);
        approx::assert_abs_diff_eq!(var5.vec[2].get::<ohm>(), 4.0, epsilon = 1e-15);
    }
}

#[test]
fn test_deserialize_from_quantity_vec_complex() {
    {
        #[derive(Deserialize, PartialEq, Debug)]
        struct VecWrapper {
            #[serde(deserialize_with = "deserialize_vec_of_quantities")]
            vec: Vec<uom::si::complex64::MagneticFluxDensity>,
        }

        let direct_deserialization = indoc! {"
            ---
            vec: [2.0 T, (3.0 + 1i) T, 4.0 T]
            "};
        let direct_de: VecWrapper = serde_yaml::from_str(direct_deserialization).unwrap();
        assert_eq!(direct_de.vec[0].value.re, 2.0);
        assert_eq!(direct_de.vec[0].value.im, 0.0);
        assert_eq!(direct_de.vec[1].value.re, 3.0);
        assert_eq!(direct_de.vec[1].value.im, 1.0);
        assert_eq!(direct_de.vec[2].value.re, 4.0);
        assert_eq!(direct_de.vec[2].value.im, 0.0);
    }
}

#[test]
fn test_deserialize_from_str_vec_unit_mismatch() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct VecWrapper {
        #[serde(deserialize_with = "deserialize_vec_of_quantities")]
        vec: Vec<MagneticFluxDensity>,
    }

    // Expect magnetic flux density, but found kilogram
    let vec = indoc! {"
    ---
    vec: '[2.0, 3.0, 4.0] kg'
    "};
    assert!(serde_yaml::from_str::<VecWrapper>(vec).is_err());
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ElectricalResistanceWrapper {
    #[serde(deserialize_with = "deserialize_quantity")]
    quantity: ElectricalResistance,
}

#[test]
fn test_deserialize_electrical_resistance() {
    {
        let string = indoc! {"
            ---
            quantity: 1/4 V/mA
            "};
        let val: ElectricalResistanceWrapper = serde_yaml::from_str(string).unwrap();
        approx::assert_abs_diff_eq!(val.quantity.get::<ohm>(), 250.0, epsilon = 1e-9);
    }
    {
        let string = indoc! {"
            ---
            quantity: PI uV/mA
            "};
        let val: ElectricalResistanceWrapper = serde_yaml::from_str(string).unwrap();
        assert_eq!(val.quantity.get::<ohm>(), PI * 1e-3);
    }
    {
        let string = indoc! {"
            ---
            quantity: PI µV/mA
            "};
        let val: ElectricalResistanceWrapper = serde_yaml::from_str(string).unwrap();
        assert_eq!(val.quantity.get::<ohm>(), PI * 1e-3);
    }
}

#[derive(Deserialize)]
struct AngleWrapper {
    #[serde(deserialize_with = "deserialize_angle")]
    val: f64,
}

#[test]
fn test_floating_point() {
    let data = indoc! {"
            ---
            val: 2.0
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, 2.0, epsilon = 1e-15);
}

#[test]
fn test_pi_conversion() {
    let data = indoc! {"
            ---
            val: 2*pi
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: 2.0*pi
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: 2.0*pi rad
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -2.0*pi
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -2.0*pi
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -2.0 *pi
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -2.0pi
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -2.0π
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -TAU, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -2.0π rad
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -TAU, epsilon = 1e-15);
}

#[test]
fn test_angle_in_degree() {
    let data = indoc! {"
            ---
            val: 90deg
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, FRAC_PI_2, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: 90 deg
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, FRAC_PI_2, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: 90.0deg
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, FRAC_PI_2, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: 90.0 deg
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, FRAC_PI_2, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -90.0deg
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -FRAC_PI_2, epsilon = 1e-15);
}

#[test]
fn test_angle_in_rad() {
    let data = indoc! {"
            ---
            val: 1rad
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, 1.0, epsilon = 1e-15);

    let data = indoc! {"
            ---
            val: -1.0rad
            "};
    let wrapper: AngleWrapper = serde_yaml::from_str(data).unwrap();
    approx::assert_abs_diff_eq!(wrapper.val, -1.0, epsilon = 1e-15);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TorqueWrapper {
    #[serde(deserialize_with = "deserialize_quantity")]
    quantity: Torque,
}

#[test]
fn test_deserialize_torque() {
    let torque = indoc! {"
        ---
        quantity: 1 mNm
        "};
    let torque: TorqueWrapper = serde_yaml::from_str(torque).unwrap();
    assert_eq!(torque.quantity.get::<newton_meter>(), 0.001);
}
