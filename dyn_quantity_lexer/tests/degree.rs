use dyn_quantity_lexer::*;

#[test]
fn test_parse_degree() {
    {
        let mut lex = Token::lexer("90 deg");

        assert_eq!(lex.next(), Some(Ok(Token::Real(90.0))));

        assert_eq!(
            lex.next(),
            Some(Ok(Token::Degree(Exponents { unit: 1, prefix: 0 })))
        );
    }
    {
        let mut lex = Token::lexer("120 deg^-2");

        assert_eq!(lex.next(), Some(Ok(Token::Real(120.0))));

        assert_eq!(
            lex.next(),
            Some(Ok(Token::Degree(Exponents {
                unit: -2,
                prefix: 0
            })))
        );
    }
    {
        let mut lex = Token::lexer("120 Degree");

        assert_eq!(lex.next(), Some(Ok(Token::Real(120.0))));

        assert_eq!(
            lex.next(),
            Some(Ok(Token::Degree(Exponents { unit: 1, prefix: 0 })))
        );
    }
    {
        let mut lex = Token::lexer("120 degree");

        assert_eq!(lex.next(), Some(Ok(Token::Real(120.0))));

        assert_eq!(
            lex.next(),
            Some(Ok(Token::Degree(Exponents { unit: 1, prefix: 0 })))
        );
    }
    {
        let mut lex = Token::lexer("120 mdegree");

        // Read out the number
        assert_eq!(lex.next(), Some(Ok(Token::Real(120.0))));

        assert_eq!(
            lex.next(),
            Some(Ok(Token::Degree(Exponents {
                unit: 1,
                prefix: -3
            })))
        );
    }
    {
        let mut lex = Token::lexer("PI/2 degree^-3");

        // Read out the number
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Pi(Exponents { unit: 1, prefix: 0 })))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Div)));
        assert_eq!(lex.next(), Some(Ok(Token::Real(2.0))));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Degree(Exponents {
                unit: -3,
                prefix: 0
            })))
        );
    }
}

#[test]
fn test_parse_rad() {
    {
        let mut lex = Token::lexer("PI/2 rad^2");

        // Read out the number
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Pi(Exponents { unit: 1, prefix: 0 })))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Div)));
        assert_eq!(lex.next(), Some(Ok(Token::Real(2.0))));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Radians(Exponents { unit: 2, prefix: 0 })))
        );
    }
}
