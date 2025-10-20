use std::str::FromStr;

use dyn_quantity::*;

#[test]
fn test_unexpected_token() {
    // We don't know what "l" is
    {
        let error = DynQuantity::<f64>::from_str("2 l").unwrap_err();
        assert_eq!(error.span.start, 2);
        assert_eq!(error.span.end, 3);
        assert_eq!(error.substring, "l");
    }
    {
        let error = DynQuantity::<f64>::from_str("2 l 3").unwrap_err();
        assert_eq!(error.span.start, 2);
        assert_eq!(error.span.end, 3);
        assert_eq!(error.substring, "l");
    }
    {
        let error = DynQuantity::<f64>::from_str("2 $ 3").unwrap_err();
        assert_eq!(error.span.start, 2);
        assert_eq!(error.span.end, 3);
        assert_eq!(error.substring, "$");
    }
}

#[test]
fn test_starts_with_bad_symbol() {
    {
        let error = DynQuantity::<f64>::from_str("*3").unwrap_err();
        match error.reason {
            ParseErrorReason::MustNotStartWith => (),
            _ => panic!("wrong error type"),
        }
        assert_eq!(error.substring, "*");
    }
    {
        let error = DynQuantity::<f64>::from_str("/3").unwrap_err();
        match error.reason {
            ParseErrorReason::MustNotStartWith => (),
            _ => panic!("wrong error type"),
        }
        assert_eq!(error.substring, "/");
    }
}

#[test]
fn test_unbalanced_brackets() {
    {
        let error = DynQuantity::<f64>::from_str("1)").unwrap_err();
        match error.reason {
            ParseErrorReason::UnbalancedBrackets => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("((2)").unwrap_err();
        match error.reason {
            ParseErrorReason::UnbalancedBrackets => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("(3))").unwrap_err();
        match error.reason {
            ParseErrorReason::UnbalancedBrackets => (),
            _ => panic!("wrong error type"),
        }
    }
}

#[test]
fn test_two_operators_without_number() {
    {
        let error = DynQuantity::<f64>::from_str("1++1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("1+-1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("1-+1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("1--1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("1-*1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("1**1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("1/*1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
    {
        let error = DynQuantity::<f64>::from_str("1//1").unwrap_err();
        match error.reason {
            ParseErrorReason::TwoOperatorsWithoutNumber => (),
            _ => panic!("wrong error type"),
        }
    }
}
