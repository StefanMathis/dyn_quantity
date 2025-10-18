/*!
This crate contains the lexer for the "dyn_quantity" crate. It is separated from
"dyn_quantity" because the lexer is created using the [logos](https://docs.rs/logos/latest/logos/)
macros. These expand into a lot of code, making it hard for language servers
such as rust-analyzer to keep up. The separation makes it possible to use
language servers together with "dyn_quantity". See the README.md of
"dyn_quantity" for further details.
*/

pub use logos::{Lexer, Logos, Span};
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq)]
pub struct Exponents {
    pub unit: i32,
    pub prefix: i32,
}

impl Exponents {
    pub fn exponent(&self) -> i32 {
        return self.unit * self.prefix;
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
#[logos(extras = (usize, usize))]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(r"(\d*)((\.)?\d+)", |lex| lex.slice().parse(), priority = 3)]
    Real(f64),

    #[regex(r"(\d*)(\.\d+)? ?i", |lex| parse_imag(lex), priority = 4)]
    #[regex(r"(\d*)(\.\d+)? ?j", |lex| parse_imag(lex), priority = 4)]
    Imag(f64),

    #[regex(r"\* ?10\^-?\d+", |lex| parse_power_of_ten(lex))]
    #[regex(r"e-?\d+", |lex| parse_power_of_ten_e(lex))]
    PowerOfTen(i32),

    #[token("inf")]
    #[token("Inf")]
    #[token("INF")]
    #[token("infinity")]
    #[token("Infinity")]
    #[token("INFINITY")]
    #[token(".inf")]
    #[token(".Inf")]
    #[token(".INF")]
    Infinity,

    #[token("-inf")]
    #[token("-Inf")]
    #[token("-INF")]
    #[token("-infinity")]
    #[token("-Infinity")]
    #[token("-INFINITY")]
    #[token("-.inf")]
    #[token("-.Inf")]
    #[token("-.INF")]
    NegInfinity,

    #[token("(")]
    LeftBracket,

    #[token(")", |lex| parse_exponent(lex))]
    #[regex(r"\)\^-?\d+", |lex| parse_exponent(lex))]
    RightBracket(i32),

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("%")]
    Percent,

    #[regex(r"[a-zA-Zµ]?s", |lex| parse_exponents_and_prefix(lex, "s"))]
    #[regex(r"[a-zA-Zµ]?s\^-?\d+", |lex| parse_exponents_and_prefix(lex, "s"))]
    Second(Exponents),

    #[regex(r"[a-zA-Zµ]?m", |lex| parse_exponents_and_prefix(lex, "m"))]
    #[regex(r"[a-zA-Zµ]?m\^-?\d+", |lex| parse_exponents_and_prefix(lex, "m"))]
    Meter(Exponents),

    #[regex(r"[a-zA-Zµ]?g", |lex| parse_exponents_and_prefix(lex, "g"))]
    #[regex(r"[a-zA-Zµ]?g\^-?\d+", |lex| parse_exponents_and_prefix(lex, "g"))]
    Gram(Exponents),

    #[regex(r"[a-zA-Zµ]?A", |lex| parse_exponents_and_prefix(lex, "A"))]
    #[regex(r"[a-zA-Zµ]?A\^-?\d+", |lex| parse_exponents_and_prefix(lex, "A"))]
    Ampere(Exponents),

    #[regex(r"[a-zA-Zµ]?K", |lex| parse_exponents_and_prefix(lex, "K"))]
    #[regex(r"[a-zA-Zµ]?K\^-?\d+", |lex| parse_exponents_and_prefix(lex, "K"))]
    Kelvin(Exponents),

    #[regex(r"[a-zA-Zµ]?mol", |lex| parse_exponents_and_prefix(lex, "mol"))]
    #[regex(r"[a-zA-Zµ]?mol\^-?\d+", |lex| parse_exponents_and_prefix(lex, "mol"))]
    Mol(Exponents),

    #[regex(r"[a-zA-Zµ]?cd", |lex| parse_exponents_and_prefix(lex, "cd"))]
    #[regex(r"[a-zA-Zµ]?cd\^-?\d+", |lex| parse_exponents_and_prefix(lex, "cd"))]
    Candela(Exponents),

    #[regex(r"[a-zA-Zµ]?°C", |lex| parse_exponents_and_prefix(lex, "°C"))]
    #[regex(r"[a-zA-Zµ]?°C\^-?\d+", |lex| parse_exponents_and_prefix(lex, "°C"))]
    Celsius(Exponents),

    #[regex(r"[a-zA-Zµ]?V", |lex| parse_exponents_and_prefix(lex, "V"))]
    #[regex(r"[a-zA-Zµ]?V\^-?\d+", |lex| parse_exponents_and_prefix(lex, "V"))]
    Volt(Exponents),

    #[regex(r"[a-zA-Zµ]?N", |lex| parse_exponents_and_prefix(lex, "N"))]
    #[regex(r"[a-zA-Zµ]?N\^-?\d+", |lex| parse_exponents_and_prefix(lex, "N"))]
    Newton(Exponents),

    #[regex(r"[a-zA-Zµ]?Nm", |lex| parse_exponents_and_prefix(lex, "Nm"))]
    #[regex(r"[a-zA-Zµ]?Nm\^-?\d+", |lex| parse_exponents_and_prefix(lex, "Nm"))]
    NewtonMeter(Exponents),

    #[regex(r"[a-zA-Zµ]?W", |lex| parse_exponents_and_prefix(lex, "W"))]
    #[regex(r"[a-zA-Zµ]?W\^-?\d+", |lex| parse_exponents_and_prefix(lex, "W"))]
    Watt(Exponents),

    #[regex(r"[a-zA-Zµ]?J", |lex| parse_exponents_and_prefix(lex, "J"))]
    #[regex(r"[a-zA-Zµ]?J\^-?\d+", |lex| parse_exponents_and_prefix(lex, "J"))]
    Joule(Exponents),

    #[regex(r"[a-zA-Zµ]?Hz", |lex| parse_exponents_and_prefix(lex, "Hz"))]
    #[regex(r"[a-zA-Zµ]?Hz\^-?\d+", |lex| parse_exponents_and_prefix(lex, "Hz"))]
    Hertz(Exponents),

    #[regex(r"[a-zA-Zµ]?rpm", |lex| parse_exponents_and_prefix(lex, "rpm"))]
    #[regex(r"[a-zA-Zµ]?rpm\^-?\d+", |lex| parse_exponents_and_prefix(lex, "rpm"))]
    RotationsPerMinute(Exponents),

    #[regex(r"[a-zA-Zµ]?Wb", |lex| parse_exponents_and_prefix(lex, "Wb"))]
    #[regex(r"[a-zA-Zµ]?Wb\^-?\d+", |lex| parse_exponents_and_prefix(lex, "Wb"))]
    Weber(Exponents),

    #[regex(r"[a-zA-Zµ]?T", |lex| parse_exponents_and_prefix(lex, "T"))]
    #[regex(r"[a-zA-Zµ]?T\^-?\d+", |lex| parse_exponents_and_prefix(lex, "T"))]
    Tesla(Exponents),

    #[regex(r"[a-zA-Zµ]?H", |lex| parse_exponents_and_prefix(lex, "H"))]
    #[regex(r"[a-zA-Zµ]?H\^-?\d+", |lex| parse_exponents_and_prefix(lex, "H"))]
    Henry(Exponents),

    #[regex(r"[a-zA-Zµ]?S", |lex| parse_exponents_and_prefix(lex, "S"))]
    #[regex(r"[a-zA-Zµ]?S\^-?\d+", |lex| parse_exponents_and_prefix(lex, "S"))]
    Siemens(Exponents),

    #[regex(r"[a-zA-Zµ]?t", |lex| parse_exponents_and_prefix(lex, "t"))]
    #[regex(r"[a-zA-Zµ]?t\^-?\d+", |lex| parse_exponents_and_prefix(lex, "t"))]
    Ton(Exponents),

    #[regex(r"[a-zA-Zµ]?Ohm", |lex| parse_exponents_and_prefix(lex, "Ohm"))]
    #[regex(r"[a-zA-Zµ]?Ohm\^-?\d+", |lex| parse_exponents_and_prefix(lex, "Ohm"))]
    #[regex(r"[a-zA-Zµ]?ohm", |lex| parse_exponents_and_prefix(lex, "ohm"))]
    #[regex(r"[a-zA-Zµ]?ohm\^-?\d+", |lex| parse_exponents_and_prefix(lex, "ohm"))]
    Ohm(Exponents),

    #[regex(r"[a-zA-Zµ]?Ω", |lex| parse_exponents_and_prefix(lex, "Ω"))]
    #[regex(r"[a-zA-Zµ]?Ω\^-?\d+", |lex| parse_exponents_and_prefix(lex, "Ω"))]
    Omega(Exponents),

    #[regex(r"[a-zA-Zµ]?(pi|π|PI|Pi)", |lex| parse_pi(lex), priority = 2) ]
    #[regex(r"[a-zA-Zµ]?(pi|π|PI|Pi)\^-?\d+", |lex| parse_pi(lex), priority = 3)]
    Pi(Exponents),

    #[regex(r"[a-zA-Zµ]?(degree|°|Degree|deg|Deg)", |lex| parse_degree(lex), priority = 2) ]
    #[regex(r"[a-zA-Zµ]?(degree|°|Degree|deg|Deg)\^-?\d+", |lex| parse_degree(lex), priority = 3)]
    Degree(Exponents),

    #[regex(r"[a-zA-Zµ]?(rad|radians|Rad|Radians)", |lex| parse_radians(lex), priority = 2) ]
    #[regex(r"[a-zA-Zµ]?(rad|radians|Rad|Radians)\^-?\d+", |lex| parse_radians(lex), priority = 3)]
    Radians(Exponents),
}

fn parse_imag(lex: &mut Lexer<Token>) -> Option<f64> {
    // An imaginary number is a number followed by (possibly) a space and then either an "i" or an "j".
    // Since we're interested in the number, the space and the "i" or "j" need to be filtered out.
    let slice = lex.slice();
    match slice.find(' ') {
        Some(byte_offset) => {
            if byte_offset == 0 {
                return Some(1.0);
            } else {
                return slice[..byte_offset].parse().ok();
            }
        }
        None => {
            if slice.len() == 1 {
                return Some(1.0);
            } else {
                // If no space is in the string, we just need to remove the last byte (which is the i or the j)
                let bytes_number = slice.len() - 1;
                return slice[..bytes_number].parse().ok();
            }
        }
    }
}

fn parse_power_of_ten(lex: &mut Lexer<Token>) -> Option<i32> {
    // A power of 10 is defined as the regex * ?10\^-?\d+. This means that we need to find the position of ^.
    match lex.slice().find('^') {
        Some(byte_offset) => lex.slice()[(byte_offset + 1)..].parse().ok(),
        None => return None,
    }
}

fn parse_power_of_ten_e(lex: &mut Lexer<Token>) -> Option<i32> {
    // Ignore the e
    lex.slice()[1..].parse().ok()
}

fn parse_pi(lex: &mut Lexer<Token>) -> Option<Exponents> {
    for candidate in ["pi", "π", "PI", "Pi"].into_iter() {
        if let Some(exp) = parse_exponents_and_prefix(lex, candidate) {
            return Some(exp);
        }
    }
    return None;
}

fn parse_degree(lex: &mut Lexer<Token>) -> Option<Exponents> {
    for candidate in ["degree", "°", "Degree", "deg", "Deg"].into_iter() {
        if let Some(exp) = parse_exponents_and_prefix(lex, candidate) {
            return Some(exp);
        }
    }
    return None;
}

fn parse_radians(lex: &mut Lexer<Token>) -> Option<Exponents> {
    for candidate in ["rad", "radians", "Rad", "Radians"].into_iter() {
        if let Some(exp) = parse_exponents_and_prefix(lex, candidate) {
            return Some(exp);
        }
    }
    return None;
}

fn parse_exponents_and_prefix(lex: &mut Lexer<Token>, unit_chars: &str) -> Option<Exponents> {
    let slice = lex.slice();

    let prefix = if has_prefix(slice, unit_chars) {
        power_from_prefix(slice)?
    } else {
        0
    };

    let exponent = parse_exponent(lex)?;
    return Some(Exponents {
        unit: exponent,
        prefix,
    });
}

fn parse_exponent(lex: &mut Lexer<Token>) -> Option<i32> {
    let slice = lex.slice();

    // Find the position of the exponent marker '^'. If it cannot be found, the unit exponent is automatically set to 1
    match slice.find('^') {
        Some(byte_offset) => {
            let byte_offset = byte_offset + 1; // 1 additional offset for '^'
            let exponent = slice[byte_offset..].parse().ok()?; // Parse the number
            return Some(exponent);
        }
        None => return Some(1),
    }
}

fn has_prefix(slice: &str, unit_chars: &str) -> bool {
    return unit_chars.chars().next() == slice.chars().skip(1).next();
}

// Prefixes are taken from https://www.bipm.org/en/measurement-units/si-prefixes
fn power_from_prefix(slice: &str) -> Option<i32> {
    match slice.chars().next()? {
        'Q' => Some(30),
        'R' => Some(27),
        'Y' => Some(24),
        'Z' => Some(21),
        'E' => Some(18),
        'P' => Some(15),
        'T' => Some(12),
        'G' => Some(9),
        'M' => Some(6),
        'k' => Some(3),
        'd' => Some(-1),
        'c' => Some(-2),
        'm' => Some(-3),
        'u' => Some(-6), // Alternative representation for µ
        'µ' => Some(-6),
        'n' => Some(-9),
        'p' => Some(-12),
        'f' => Some(-15),
        'a' => Some(-18),
        'z' => Some(-21),
        'y' => Some(-24),
        'r' => Some(-27),
        'q' => Some(-30),
        _ => None,
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidFloat(String),
    InvalidInt(String),
    #[default]
    CouldNotParse,
}

impl From<ParseFloatError> for LexingError {
    fn from(err: ParseFloatError) -> Self {
        return LexingError::InvalidFloat(err.to_string());
    }
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        return LexingError::InvalidInt(err.to_string());
    }
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::InvalidFloat(string) => {
                write!(f, "could not interpret {string} as a floating-point number")
            }
            LexingError::InvalidInt(string) => {
                write!(f, "could not interpret {string} as a signed integer")
            }
            LexingError::CouldNotParse => write!(f, "could not parse the input"),
        }
    }
}

impl std::error::Error for LexingError {}
