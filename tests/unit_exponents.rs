use dyn_quantity::UnitExponents;

#[test]
fn constructors() {
    {
        let exp = UnitExponents::time();
        assert_eq!(exp.second, 1);
        assert_eq!(exp.meter, 0);
        assert_eq!(exp.kilogram, 0);
        assert_eq!(exp.ampere, 0);
        assert_eq!(exp.kelvin, 0);
        assert_eq!(exp.mol, 0);
        assert_eq!(exp.candela, 0);
    }
    {
        let exp = UnitExponents::length();
        assert_eq!(exp.second, 0);
        assert_eq!(exp.meter, 1);
        assert_eq!(exp.kilogram, 0);
        assert_eq!(exp.ampere, 0);
        assert_eq!(exp.kelvin, 0);
        assert_eq!(exp.mol, 0);
        assert_eq!(exp.candela, 0);
    }
    {
        let exp = UnitExponents::mass();
        assert_eq!(exp.second, 0);
        assert_eq!(exp.meter, 0);
        assert_eq!(exp.kilogram, 1);
        assert_eq!(exp.ampere, 0);
        assert_eq!(exp.kelvin, 0);
        assert_eq!(exp.mol, 0);
        assert_eq!(exp.candela, 0);
    }
    {
        let exp = UnitExponents::electrical_current();
        assert_eq!(exp.second, 0);
        assert_eq!(exp.meter, 0);
        assert_eq!(exp.kilogram, 0);
        assert_eq!(exp.ampere, 1);
        assert_eq!(exp.kelvin, 0);
        assert_eq!(exp.mol, 0);
        assert_eq!(exp.candela, 0);
    }
    {
        let exp = UnitExponents::temperature();
        assert_eq!(exp.second, 0);
        assert_eq!(exp.meter, 0);
        assert_eq!(exp.kilogram, 0);
        assert_eq!(exp.ampere, 0);
        assert_eq!(exp.kelvin, 1);
        assert_eq!(exp.mol, 0);
        assert_eq!(exp.candela, 0);
    }
    {
        let exp = UnitExponents::amount_of_substance();
        assert_eq!(exp.second, 0);
        assert_eq!(exp.meter, 0);
        assert_eq!(exp.kilogram, 0);
        assert_eq!(exp.ampere, 0);
        assert_eq!(exp.kelvin, 0);
        assert_eq!(exp.mol, 1);
        assert_eq!(exp.candela, 0);
    }
    {
        let exp = UnitExponents::luminous_intensity();
        assert_eq!(exp.second, 0);
        assert_eq!(exp.meter, 0);
        assert_eq!(exp.kilogram, 0);
        assert_eq!(exp.ampere, 0);
        assert_eq!(exp.kelvin, 0);
        assert_eq!(exp.mol, 0);
        assert_eq!(exp.candela, 1);
    }
}
