use dyn_quantity::{PredefUnit, Unit};

#[test]
fn test_multiplication() {
    {
        let first: Unit = PredefUnit::ElectricCurrent.into();
        let second: Unit = PredefUnit::Length.into();
        let result = first * second;
        assert_eq!(result.ampere, 1);
        assert_eq!(result.meter, 1);
    }
}

#[test]
fn test_division() {
    {
        let first: Unit = PredefUnit::ElectricCurrent.into();
        let second: Unit = PredefUnit::Length.into();
        let result = first / second;
        assert_eq!(result.ampere, 1);
        assert_eq!(result.meter, -1);
    }
}

#[test]
fn test_serde() {
    {
        let unit: Unit = PredefUnit::ElectricCurrent.into();
        assert_eq!(unit.ampere, 1);
        let str = serde_yaml::to_string(&unit).unwrap();
        let de_unit: Unit = serde_yaml::from_str(&str).unwrap();
        assert_eq!(de_unit, unit);
    }
}

#[test]
fn test_deserialize_from_common_units() {
    #[derive(Debug, serde::Deserialize)]
    struct UnitWrapper {
        field: Unit,
    }
    {
        let str = "---\nfield: Area";
        let unit: UnitWrapper = serde_yaml::from_str(str).unwrap();
        assert_eq!(unit.field.meter, 2);
    }
    {
        let str = "---\nfield: ElectricCurrent";
        let unit: UnitWrapper = serde_yaml::from_str(str).unwrap();
        assert_eq!(unit.field.ampere, 1);
    }
}
