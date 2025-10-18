use ::num::{Complex, Zero};
use dyn_quantity_lexer::{Logos, Token};
use std::{
    f64::{INFINITY, NEG_INFINITY, consts::PI},
    str::FromStr,
};

use crate::*;

impl<V: F64RealOrComplex> FromStr for DynQuantity<V> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dyn_quantity = from_str_complexf64(s)?;
        match V::try_from_complexf64(dyn_quantity.value) {
            Ok(value) => {
                return Ok(DynQuantity::new(value, dyn_quantity.exponents));
            }
            Err(conversion_error) => {
                return Err(ParseError {
                    substring: "".into(),
                    span: 0..0,
                    reason: ParseErrorReason::NotConvertibleFromComplexF64(conversion_error),
                });
            }
        }
    }
}

fn from_str_complexf64(s: &str) -> Result<DynQuantity<Complex<f64>>, ParseError> {
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum PreviousToken {
        Add,
        Sub,
        Mul,
        Div,
        Other,
    }

    /**
    This enum encapsulates a quantity and the mathematical operation which follows it.
    For example:
    Add(x) means "x +", Mul(x) means "x *", Div(x) means "x /".
     */
    #[derive(Debug)]
    enum Operation {
        Add(DynQuantity<Complex<f64>>),
        Mul(DynQuantity<Complex<f64>>),
        Div(DynQuantity<Complex<f64>>),
    }

    impl From<Operation> for DynQuantity<Complex<f64>> {
        fn from(value: Operation) -> Self {
            match value {
                Operation::Add(item) => item,
                Operation::Mul(item) => item,
                Operation::Div(item) => item,
            }
        }
    }

    fn adjust<F: FnMut(&mut DynQuantity<Complex<f64>>)>(
        active_quantity: &mut Option<DynQuantity<Complex<f64>>>,
        mut fun: F,
    ) {
        let mut quantity = active_quantity
            .take()
            .unwrap_or(DynQuantity::new(Complex::new(1.0, 0.0), Default::default()));

        fun(&mut quantity);

        *active_quantity = Some(quantity);
    }

    fn include_infinity(active_quantity: &mut Option<DynQuantity<Complex<f64>>>, infinity: f64) {
        if let Some(quantity) = active_quantity.as_mut() {
            let re = if quantity.value.re == 0.0 {
                0.0
            } else {
                quantity.value.re.signum() * infinity
            };
            let im = if quantity.value.im == 0.0 {
                0.0
            } else {
                quantity.value.re.signum() * infinity
            };
            quantity.value = Complex::new(re, im);
        } else {
            *active_quantity = Some(DynQuantity::new(
                Complex::new(infinity, 0.0),
                Default::default(),
            ));
        }
    }

    /**
    When multiplying an infinite value with zero, the IEEE result is NaN.
    However, in our case the result should be 0
     */
    fn multiply_no_nan(arg1: Complex<f64>, arg2: Complex<f64>) -> Complex<f64> {
        // Multiply "by hand" in order to treat 0 * Inf accordingly
        let mut re = 0.0;
        let mut im = 0.0;

        if !(arg1.re.is_infinite() && arg2.re.is_zero())
            && !(arg1.re.is_zero() && arg2.re.is_infinite())
        {
            re += arg1.re * arg2.re;
        }

        if !(arg1.im.is_infinite() && arg2.re.is_zero())
            && !(arg1.im.is_zero() && arg2.re.is_infinite())
        {
            im += arg1.im * arg2.re;
        }

        if !(arg1.re.is_infinite() && arg2.im.is_zero())
            && !(arg1.re.is_zero() && arg2.im.is_infinite())
        {
            im += arg1.re * arg2.im;
        }

        if !(arg1.im.is_infinite() && arg2.im.is_zero())
            && !(arg1.im.is_zero() && arg2.im.is_infinite())
        {
            re -= arg1.im * arg2.im;
        }

        return Complex::new(re, im);
    }

    // ===============================================================================

    let mut lexer = Token::lexer(s);

    let mut active_quantity: Option<DynQuantity<Complex<f64>>> = None;

    // This is a stack of quantities. Two quantities are separated by a mathematical operator
    // which defines how the quantities are combined. THe last operator is combined with "active_quantity".
    // For example, stack = [Add(x), Mul(y)] and active_quantity = Some(z), where x, y and z are quantities would
    // be combined as follows:
    // x + y * z
    let mut stack: Vec<Operation> = Vec::new();

    // The bracket level increases by one for each open bracket "(" and decreases by one for each closing
    // bracket ")". At the end of the parsing, the bracket level must be zero. Additionally, it may never become
    // negative, therefore we always use checked_sub. If it would become negative or is not zero at the end,
    // an error is returned.
    let mut bracket_level: usize = 0;

    let mut previous_token = PreviousToken::Other;
    let mut division_pending = false;

    while let Some(token) = lexer.next() {
        let token: Token = token.map_err(|_| {
            let reason = ParseErrorReason::UnexpectedToken;
            return ParseError {
                substring: s[lexer.span()].to_owned(),
                span: lexer.span(),
                reason,
            };
        })?;

        match token {
            Token::Real(val) => {
                if let Some(quantity) = active_quantity.as_mut() {
                    quantity.value = multiply_no_nan(quantity.value, Complex::new(val, 0.0));
                } else {
                    active_quantity =
                        Some(DynQuantity::new(Complex::new(val, 0.0), Default::default()));
                }
            }
            Token::Imag(val) => {
                if let Some(quantity) = active_quantity.as_mut() {
                    quantity.value = multiply_no_nan(quantity.value, Complex::new(0.0, val));
                } else {
                    active_quantity =
                        Some(DynQuantity::new(Complex::new(0.0, val), Default::default()));
                }
            }
            Token::Infinity => {
                include_infinity(&mut active_quantity, INFINITY);
            }
            Token::NegInfinity => {
                include_infinity(&mut active_quantity, NEG_INFINITY);
            }
            Token::Mul => {
                // This is essentially a no-op - we therefore do just some error checking
                if previous_token != PreviousToken::Other {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                if active_quantity.is_none() {
                    let reason = ParseErrorReason::MustNotStartWith;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                previous_token = PreviousToken::Mul;
                continue;
            }
            Token::Div => {
                if previous_token != PreviousToken::Other {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                if let Some(quantity) = active_quantity.take() {
                    stack.push(Operation::Div(quantity));
                } else {
                    let reason = ParseErrorReason::MustNotStartWith;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
                previous_token = PreviousToken::Div;
                division_pending = true;

                // Short-circuit to retain previous_token and division_pending into the next division
                continue;
            }
            Token::Percent => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *= 1e-2;
                });
            }
            Token::Pi(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *= PI.powi(exponents.unit) * 10.0.powi(exponents.exponent());
                });
            }
            Token::LeftBracket => {
                if let Some(quantity) = active_quantity.take() {
                    stack.push(Operation::Mul(quantity));
                } else {
                    /*
                    If a bracket starts and the previous token was not an operator, add
                    a buffer operation (multiplication with one) to the stack so the stack popping
                    stops at the opening bracket when the closing (right) bracket comes.
                     */
                    if previous_token == PreviousToken::Other {
                        stack.push(Operation::Mul(DynQuantity::new(
                            Complex::new(1.0, 0.0),
                            Default::default(),
                        )));
                    }
                }
                bracket_level += 1;
            }
            Token::RightBracket(exponent) => match bracket_level.checked_sub(1) {
                // Merge all stack elements with the current bracket up to and including the first "multiply".
                Some(val) => {
                    if let Some(mut quantity) = active_quantity.take() {
                        while let Some(stack_item) = stack.pop() {
                            match stack_item {
                                Operation::Add(elem) => {
                                    quantity.try_add_assign(&elem).map_err(|add| {
                                        return ParseError {
                                            substring: s[lexer.span()].to_owned(),
                                            span: lexer.span(),
                                            reason: ParseErrorReason::UnitsOfSummandsNotIdentical(
                                                add,
                                            ),
                                        };
                                    })?
                                }
                                Operation::Mul(elem) => {
                                    quantity = elem * quantity.powi(exponent);
                                    break;
                                }
                                Operation::Div(elem) => {
                                    quantity = elem / quantity.powi(exponent);
                                    break;
                                }
                            }
                        }

                        // The resolved bracket becomes the new active quantity
                        active_quantity = Some(quantity);

                        // Adjust the bracket level
                        bracket_level = val;
                    } else {
                        let reason = ParseErrorReason::UnbalancedBrackets;
                        return Err(ParseError {
                            substring: s[lexer.span()].to_owned(),
                            span: lexer.span(),
                            reason,
                        });
                    }
                }
                None => {
                    let reason = ParseErrorReason::UnbalancedBrackets;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }
            },
            Token::Add => {
                // Constructs such as "-+" or "++" are not allowed, but *+1 is
                if previous_token == PreviousToken::Add || previous_token == PreviousToken::Sub {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }

                let new_quantity = DynQuantity::new(Complex::new(1.0, 0.0), Default::default());
                if let Some(quantity) = active_quantity.replace(new_quantity) {
                    stack.push(Operation::Add(quantity));
                }
                previous_token = PreviousToken::Add;
                continue;
            }
            Token::Sub => {
                // Constructs such as "+-" or "--" are not allowed, but /-1 or /+1 is
                if previous_token == PreviousToken::Add || previous_token == PreviousToken::Sub {
                    let reason = ParseErrorReason::TwoOperatorsWithoutNumber;
                    return Err(ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason,
                    });
                }

                let new_quantity = DynQuantity::new(Complex::new(-1.0, 0.0), Default::default());
                if let Some(quantity) = active_quantity.replace(new_quantity) {
                    stack.push(Operation::Add(quantity));
                }
                previous_token = PreviousToken::Sub;
                continue;
            }
            Token::PowerOfTen(exponent) => {
                if let Some(quantity) = active_quantity.as_mut() {
                    quantity.value *= 10.0.powi(exponent);
                } else {
                    active_quantity = Some(DynQuantity::new(
                        Complex::new(10.0.powi(exponent), 0.0),
                        Default::default(),
                    ));
                }
            }
            Token::Second(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.second += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Meter(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.meter += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Gram(mut exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    // Special treatment of gram: The prefix needs to be reduced by 3, since the SI system works in kilogram
                    exponents.prefix -= 3;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Ampere(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.ampere += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Kelvin(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kelvin += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Mol(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.mol += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Candela(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.candela += exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Celsius(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kelvin += exponents.unit;
                    // Special treatment of celsius: The value needs to be corrected by an offset of -273.15 to the power of the unit exponent
                    quantity.value += (273.15f64).powi(exponents.unit);
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Newton(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Watt(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 3 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Joule(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Volt(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.ampere -= exponents.unit;
                    quantity.exponents.second -= 3 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Weber(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.ampere -= exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Tesla(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.ampere -= exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Henry(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.ampere -= 2 * exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Hertz(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.second -= exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Siemens(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram -= exponents.unit;
                    quantity.exponents.meter -= 2 * exponents.unit;
                    quantity.exponents.second += 3 * exponents.unit;
                    quantity.exponents.ampere += 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::RotationsPerMinute(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.second -= exponents.unit;
                    quantity.value *=
                        (1.0f64 / 60.0f64).powi(exponents.unit) * 10.0.powi(exponents.exponent());
                });
            }
            Token::Degree(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *=
                        (PI / 180.0).powi(exponents.unit) * 10.0.powi(exponents.exponent());
                });
            }
            Token::Radians(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Ohm(exponents) | Token::Omega(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 3 * exponents.unit;
                    quantity.exponents.ampere -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::NewtonMeter(exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    quantity.exponents.meter += 2 * exponents.unit;
                    quantity.exponents.second -= 2 * exponents.unit;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
            Token::Ton(mut exponents) => {
                adjust(&mut active_quantity, |quantity| {
                    quantity.exponents.kilogram += exponents.unit;
                    // Special treatment of gram: The prefix needs to be increased by 3
                    exponents.prefix += 3;
                    quantity.value *= 10.0.powi(exponents.exponent());
                });
            }
        }

        // If the last element of the stack is a division and the next token was no open bracket,
        // perform the division immediately
        if division_pending {
            if let Some(last_stack_item) = stack.last() {
                if let Operation::Div(_) = last_stack_item {
                    if let Some(quantity) = active_quantity.take() {
                        // Remove the last element of the stack, so it can be used in the division
                        let popped_quantity: DynQuantity<Complex<f64>> =
                            stack.pop().expect("stack has at least one element").into();

                        // Perform the division
                        active_quantity = Some(popped_quantity / quantity);
                    }
                }
            } else {
                // Division without stack item would mean that the string looks something like this: "/3"
                // This results in a parse error
                let reason = ParseErrorReason::UnbalancedBrackets;
                return Err(ParseError {
                    substring: s[lexer.span()].to_owned(),
                    span: lexer.span(),
                    reason,
                });
            }
        }
        division_pending = false;
        previous_token = PreviousToken::Other;
    }
    // End of the loop

    // Check the bracket level
    if bracket_level != 0 {
        let reason = ParseErrorReason::UnbalancedBrackets;
        return Err(ParseError {
            substring: s[lexer.span()].to_owned(),
            span: lexer.span(),
            reason,
        });
    }

    if let Some(initial) = stack.pop() {
        let initial = if let Some(quantity) = active_quantity.take() {
            stack.push(initial);
            quantity
        } else {
            initial.into()
        };

        stack.into_iter().try_fold(initial, |mut acc, item| {
            match item {
                Operation::Add(item) => acc.try_add_assign(&item).map_err(|add| {
                    return ParseError {
                        substring: s[lexer.span()].to_owned(),
                        span: lexer.span(),
                        reason: ParseErrorReason::UnitsOfSummandsNotIdentical(add),
                    };
                })?,
                Operation::Mul(item) => {
                    acc = item * acc;
                }
                Operation::Div(item) => {
                    acc = item / acc;
                }
            }
            Ok(acc)
        })
    } else {
        if let Some(quantity) = active_quantity.take() {
            return Ok(quantity);
        } else {
            let reason = ParseErrorReason::InputIsEmpty;
            return Err(ParseError {
                substring: s[lexer.span()].to_owned(),
                span: lexer.span(),
                reason,
            });
        }
    }
}
