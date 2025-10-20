use dyn_quantity::*;
use indoc::indoc;
use num::Complex;
use serde::Deserialize;

#[test]
fn test_deserialize_from_string() {
    {
        let quantity: DynQuantity<f64> = serde_yaml::from_str("1 mA").unwrap();
        assert_eq!(quantity.value, 1e-3);
        assert_eq!(quantity.unit.ampere, 1);
    }
    {
        let quantity: DynQuantity<Complex<f64>> = serde_yaml::from_str("(1+2i) kg^2").unwrap();
        assert_eq!(quantity.value, Complex::new(1.0, 2.0));
        assert_eq!(quantity.unit.kilogram, 2);
    }
}

#[test]
fn test_deserialize_and_serialize() {
    {
        let quantity: DynQuantity<f64> = serde_yaml::from_str("2 A*m").unwrap();
        let res = serde_yaml::to_string(&quantity).unwrap();
        let expected = indoc! {"
        ---
        value: 2.0
        exponents:
          second: 0
          meter: 1
          kilogram: 0
          ampere: 1
          kelvin: 0
          mol: 0
          candela: 0
        "};
        assert_eq!(&res, &expected);
    }
    {
        let quantity: DynQuantity<Complex<f64>> = serde_yaml::from_str("(1+2i) kg^2").unwrap();
        let res = serde_yaml::to_string(&quantity).unwrap();
        assert_eq!(
            &res,
            "---\nvalue:\n  - 1.0\n  - 2.0\nexponents:\n  second: 0\n  meter: 0\n  kilogram: 2\n  ampere: 0\n  kelvin: 0\n  mol: 0\n  candela: 0\n"
        );
    }
}

#[test]
fn test_deserialize_vec_dyn_quantity() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct VecWrapper {
        #[serde(deserialize_with = "deserialize_vec_of_quantities")]
        vec: Vec<DynQuantity<f64>>,
    }

    {
        let no_units = indoc! {"
        ---
        vec: [2.0, 3.0, 4.0]
        "};
        let wrapper: VecWrapper = serde_yaml::from_str(no_units).unwrap();
        assert_eq!(wrapper.vec[0].value, 2.0);
        assert_eq!(wrapper.vec[1].value, 3.0);
        assert_eq!(wrapper.vec[2].value, 4.0);
    }
    {
        let vec_of_quantites = indoc! {"
        ---
        vec: [2.0 T, 3.0 T, 4.0 T]
        "};
        let wrapper: VecWrapper = serde_yaml::from_str(vec_of_quantites).unwrap();
        assert_eq!(wrapper.vec[0].value, 2.0);
        assert_eq!(wrapper.vec[1].value, 3.0);
        assert_eq!(wrapper.vec[2].value, 4.0);
    }
    {
        let quantity_vec = indoc! {"
        ---
        vec: '[2.0, 3.0, 4.0] T'
        "};
        let wrapper: VecWrapper = serde_yaml::from_str(quantity_vec).unwrap();
        assert_eq!(wrapper.vec[0].value, 2.0);
        assert_eq!(wrapper.vec[1].value, 3.0);
        assert_eq!(wrapper.vec[2].value, 4.0);
    }
}
