use std::str::FromStr;

use dyn_quantity::*;
use num::Complex;

#[test]
fn test_display() {
    {
        let quantity = DynQuantity::<f64>::from_str("1e3 m^-2").unwrap();
        assert_eq!(&quantity.to_string(), "1000 m^-2");
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 mm^-2").unwrap();
        assert_eq!(&quantity.to_string(), "1000000 m^-2");
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 ms^2").unwrap();
        assert_eq!(&quantity.to_string(), "0.000001 s^2");
    }
    {
        let quantity = DynQuantity::<f64>::from_str("1 kg^2").unwrap();
        assert_eq!(&quantity.to_string(), "1 kg^2");
    }
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("(1 + 2i) A").unwrap();
        assert_eq!(&quantity.to_string(), "(1+2i) A");
    }
    {
        let quantity = DynQuantity::<Complex<f64>>::from_str("(1 + 2j) A").unwrap();
        assert_eq!(&quantity.to_string(), "(1+2i) A");
    }
}
