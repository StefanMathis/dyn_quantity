use dyn_quantity_lexer::*;

#[test]
fn test_parse_arithmetic_operators() {
    let mut lex = Token::lexer("-+");

    // Read out the number
    assert_eq!(lex.next(), Some(Ok(Token::Sub)));
    assert_eq!(lex.next(), Some(Ok(Token::Add)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_parse_exponent() {
    {
        let mut lex = Token::lexer(")");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::RightBracket(1))));
        assert_eq!(lex.next(), None);
    }
    {
        let mut lex = Token::lexer(")^2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::RightBracket(2))));
        assert_eq!(lex.next(), None);
    }
    {
        let mut lex = Token::lexer(")^-2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::RightBracket(-2))));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_parse_time() {
    {
        let mut lex = Token::lexer(".9 s");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Second(Exponents { unit: 1, prefix: 0 })))
        );
    }

    {
        let mut lex = Token::lexer("0.9 s^-2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Second(Exponents {
                unit: -2,
                prefix: 0
            })))
        );
    }

    {
        let mut lex = Token::lexer("9 ms^3");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(9.0))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Second(Exponents {
                unit: 3,
                prefix: -3
            })))
        );
    }

    {
        let mut lex = Token::lexer("9 ks^-1");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(9.0))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Second(Exponents {
                unit: -1,
                prefix: 3
            })))
        );
    }

    {
        let mut lex = Token::lexer("ks^-1");

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Second(Exponents {
                unit: -1,
                prefix: 3
            })))
        );
    }
}

#[test]
fn test_parse_distance() {
    {
        let mut lex = Token::lexer(".9 mm");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Meter(Exponents {
                unit: 1,
                prefix: -3
            })))
        );
    }

    {
        let mut lex = Token::lexer(".9 km^2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Meter(Exponents { unit: 2, prefix: 3 })))
        );
    }
}

#[test]
fn test_parse_weight() {
    {
        let mut lex = Token::lexer(".9 kg");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Gram(Exponents { unit: 1, prefix: 3 })))
        );
    }
    {
        let mut lex = Token::lexer(".9 mg^2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Gram(Exponents {
                unit: 2,
                prefix: -3
            })))
        );
    }
    {
        let mut lex = Token::lexer(".9 ug^2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Gram(Exponents {
                unit: 2,
                prefix: -6
            })))
        );
    }
}

#[test]
fn test_parse_current() {
    {
        let mut lex = Token::lexer(".9 mA^-1");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Ampere(Exponents {
                unit: -1,
                prefix: -3
            })))
        );
    }
}

#[test]
fn test_parse_combined() {
    {
        let mut lex = Token::lexer(".9 mm/s^2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        assert_eq!(
            lex.next(),
            Some(Ok(Token::Meter(Exponents {
                unit: 1,
                prefix: -3
            })))
        );

        assert_eq!(lex.next(), Some(Ok(Token::Div)));

        assert_eq!(
            lex.next(),
            Some(Ok(Token::Second(Exponents { unit: 2, prefix: 0 })))
        );
    }
}

#[test]
fn test_parse_magnetic_flux() {
    {
        let mut lex = Token::lexer(".9 mWb");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Weber(Exponents {
                unit: 1,
                prefix: -3
            })))
        );
    }
}

#[test]
fn test_parse_temperature() {
    {
        let mut lex = Token::lexer(".9 m°C");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Celsius(Exponents {
                unit: 1,
                prefix: -3
            })))
        );
    }
}

#[test]
fn test_parse_torque() {
    {
        let mut lex = Token::lexer("2 Nm");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(2.0))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::NewtonMeter(Exponents { unit: 1, prefix: 0 })))
        );
    }
}

#[test]
fn test_parse_greek_unit() {
    {
        let mut lex = Token::lexer(".9 Ω");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Omega(Exponents { unit: 1, prefix: 0 })))
        );
    }
}

#[test]
fn test_parse_greek_prefix() {
    {
        let mut lex = Token::lexer(".9 µg^2");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(0.9))));

        // Read the exponent of the time
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Gram(Exponents {
                unit: 2,
                prefix: -6
            })))
        );
    }
}

#[test]
fn test_parse_imag() {
    {
        let mut lex = Token::lexer(".9 i");
        assert_eq!(lex.next(), Some(Ok(Token::Imag(0.9))));
    }
    {
        let mut lex = Token::lexer("-.9i");
        assert_eq!(lex.next(), Some(Ok(Token::Sub)));
        assert_eq!(lex.next(), Some(Ok(Token::Imag(0.9))));
    }
    {
        let mut lex = Token::lexer("-.9 i");
        assert_eq!(lex.next(), Some(Ok(Token::Sub)));
        assert_eq!(lex.next(), Some(Ok(Token::Imag(0.9))));
    }
    {
        let mut lex = Token::lexer("-.9 j");
        assert_eq!(lex.next(), Some(Ok(Token::Sub)));
        assert_eq!(lex.next(), Some(Ok(Token::Imag(0.9))));
    }
    {
        let mut lex = Token::lexer(".9j");
        assert_eq!(lex.next(), Some(Ok(Token::Imag(0.9))));
    }
    {
        let mut lex = Token::lexer("50j");
        assert_eq!(lex.next(), Some(Ok(Token::Imag(50.0))));
    }
    {
        let mut lex = Token::lexer("-50j");
        assert_eq!(lex.next(), Some(Ok(Token::Sub)));
        assert_eq!(lex.next(), Some(Ok(Token::Imag(50.0))));
    }
    {
        let mut lex = Token::lexer("i");
        assert_eq!(lex.next(), Some(Ok(Token::Imag(1.0))));
    }
    {
        let mut lex = Token::lexer("j");
        assert_eq!(lex.next(), Some(Ok(Token::Imag(1.0))));
    }
    {
        let mut lex = Token::lexer(" i");
        assert_eq!(lex.next(), Some(Ok(Token::Imag(1.0))));
    }
    {
        let mut lex = Token::lexer(" j");
        assert_eq!(lex.next(), Some(Ok(Token::Imag(1.0))));
    }
}

#[test]
fn test_parse_power_of_10() {
    {
        let mut lex = Token::lexer("*10^-2");
        assert_eq!(lex.next(), Some(Ok(Token::PowerOfTen(-2))));
    }
    {
        let mut lex = Token::lexer("* 10^2");
        assert_eq!(lex.next(), Some(Ok(Token::PowerOfTen(2))));
    }
    {
        let mut lex = Token::lexer("e-3");
        assert_eq!(lex.next(), Some(Ok(Token::PowerOfTen(-3))));
    }
    {
        let mut lex = Token::lexer("e12");
        assert_eq!(lex.next(), Some(Ok(Token::PowerOfTen(12))));
    }
}

#[test]
fn test_parse_infinite() {
    {
        let mut lex = Token::lexer("inf A");

        assert_eq!(lex.next(), Some(Ok(Token::Infinity)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Ampere(Exponents { unit: 1, prefix: 0 })))
        );
        assert_eq!(lex.next(), None);
    }
    {
        let mut lex = Token::lexer(".inf A");

        assert_eq!(lex.next(), Some(Ok(Token::Infinity)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Ampere(Exponents { unit: 1, prefix: 0 })))
        );
        assert_eq!(lex.next(), None);
    }
    {
        let mut lex = Token::lexer("-inf A");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::NegInfinity)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Ampere(Exponents { unit: 1, prefix: 0 })))
        );
        assert_eq!(lex.next(), None);
    }
    {
        let mut lex = Token::lexer("-.inf A");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::NegInfinity)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Ampere(Exponents { unit: 1, prefix: 0 })))
        );
        assert_eq!(lex.next(), None);
    }
}
