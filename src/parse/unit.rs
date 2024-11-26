use itertools::{structs::TupleWindows, Itertools};
use std::{iter, usize};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FormatUnitError {
    #[error("no unit supplied")]
    EmptyUnit,
    #[error("missing unit")]
    MissingUnit,
    #[error("an exponentiation indicator '^' can only directly follow an unit")]
    BadExpIndicatorPosition,
    #[error("bad sign position")]
    BadSignPosition,
}

/// Formats an unit to use proper notation.
pub fn format_unit(unit: &str) -> Result<String, FormatUnitError> {
    const OPS: &str = "^+⁺-⁻";
    let unit = unit.trim();
    if unit.len() == 0 {
        return Err(FormatUnitError::EmptyUnit);
    }
    let mut chars = unit.chars().peekable();
    let first_char = *chars.peek().expect("unit length has been checked");
    if "^+⁺-⁻".contains(first_char) {
        return Err(FormatUnitError::MissingUnit);
    }
    Ok(iter::once(Ok(first_char))
        .chain(
            chars
                .chain(iter::once(' '))
                .tuple_windows()
                .filter(|(b, c, a)| *c != ' ' || *a != ' ')
                .filter_map(|(c1, c2, c3)| match (c1, c2, c3) {
                    (c, '^', _) if !c.is_ascii_digit() && !"+⁺-⁻".contains(c) => None,
                    (_, '^', _) => Some(Err(FormatUnitError::BadExpIndicatorPosition)),
                    (c, '+', _) if !c.is_ascii_digit() => None,
                    (_, '+', _) => Some(Err(FormatUnitError::BadSignPosition)),
                    (c, '⁺', _) if !c.is_ascii_digit() => None,
                    (_, '⁺', _) => Some(Err(FormatUnitError::BadSignPosition)),
                    (c, '-', _) if !c.is_ascii_digit() => Some(Ok('⁻')),
                    (_, '-', _) => Some(Err(FormatUnitError::BadSignPosition)),
                    (c, '⁻', _) if !c.is_ascii_digit() => Some(Ok('⁻')),
                    (_, '⁻', _) => Some(Err(FormatUnitError::BadSignPosition)),
                    (_, ' ', a) if a.is_numeric() || OPS.contains(a) => None,
                    (c, '-', _) => Some(Ok('⁻')),
                    (_, '*', _) => Some(Ok('·')),
                    (_, '.', _) => Some(Ok('⋅')),
                    _ => Some(Ok(c2.to_superscript_digit().unwrap_or(c2))),
                }),
        )
        .collect::<Result<_, _>>()?)
}

#[derive(Error, Debug, PartialEq)]
#[error("not a valid digit to superscript")]
pub struct NotADigit;

pub trait CharUtils {
    fn to_superscript_digit(self) -> Result<char, NotADigit>;
}

impl CharUtils for char {
    fn to_superscript_digit(self) -> Result<char, NotADigit> {
        digit_char_to_superscript(self)
    }
}

/// Converts a digit character to its superscript form.
pub const fn digit_char_to_superscript(d: char) -> Result<char, NotADigit> {
    const SUPER_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    if let Some(d) = d.to_digit(10) {
        Ok(SUPER_DIGITS[d as usize])
    } else {
        Err(NotADigit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod digit_char_to_superscript {
        use super::*;

        #[test]
        fn valid_digit() {
            assert_eq!(digit_char_to_superscript('0'), Ok('⁰'));
            assert_eq!(digit_char_to_superscript('1'), Ok('¹'));
            assert_eq!(digit_char_to_superscript('2'), Ok('²'));
            assert_eq!(digit_char_to_superscript('3'), Ok('³'));
            assert_eq!(digit_char_to_superscript('4'), Ok('⁴'));
            assert_eq!(digit_char_to_superscript('5'), Ok('⁵'));
            assert_eq!(digit_char_to_superscript('6'), Ok('⁶'));
            assert_eq!(digit_char_to_superscript('7'), Ok('⁷'));
            assert_eq!(digit_char_to_superscript('8'), Ok('⁸'));
            assert_eq!(digit_char_to_superscript('9'), Ok('⁹'));
        }

        #[test]
        fn invalid_digit() {
            assert!(digit_char_to_superscript('⁹').is_err());
            assert!(digit_char_to_superscript('a').is_err());
        }
    }

    mod format_unit {
        use super::*;
        fn ok(s: &str) -> Result<String, FormatUnitError> {
            Ok(String::from(s))
        }

        #[test]
        fn empty_string() {
            // Placeholder test
            assert_eq!(format_unit(""), Err(FormatUnitError::EmptyUnit));
            assert_eq!(
                format_unit("     \n \r\t   "),
                Err(FormatUnitError::EmptyUnit)
            );
        }

        #[test]
        fn valid_unit() {
            assert_eq!(format_unit("m"), ok("m"));
            assert_eq!(format_unit("m²"), ok("m²"));
        }

        #[test]
        fn bad_format_unit() {
            assert_eq!(format_unit("m^2"), ok("m²"));
        }
        #[test]
        fn neg_exponent() {
            assert_eq!(format_unit("m^-1"), ok("m⁻¹"));
            assert_eq!(format_unit("m-1"), ok("m⁻¹"));
        }
        #[test]
        fn pos_exponent() {
            assert_eq!(format_unit("m^+1"), ok("m¹"));
            assert_eq!(format_unit("m+1"), ok("m¹"));
        }

        #[test]
        fn missing_unit() {
            assert_eq!(format_unit("^2"), Err(FormatUnitError::MissingUnit));
        }
        #[test]
        fn bad_exp_position() {
            assert_eq!(
                format_unit("m^2^4"),
                Err(FormatUnitError::BadExpIndicatorPosition)
            );
            assert_eq!(
                format_unit("m-^-4"),
                Err(FormatUnitError::BadExpIndicatorPosition)
            );
            assert_eq!(
                format_unit("m^+^4"),
                Err(FormatUnitError::BadExpIndicatorPosition)
            );
        }
        #[test]
        fn bad_sign_position() {
            assert_eq!(format_unit("m+2+4"), Err(FormatUnitError::BadSignPosition));
            assert_eq!(format_unit("m-2-4"), Err(FormatUnitError::BadSignPosition));
        }
        #[test]
        fn good_sign_position() {
            assert_eq!(format_unit("m^-1"), ok("m⁻¹"))
        }
        #[test]
        fn valid_complex_units() {
            assert_eq!(format_unit("m.kg^2/s-2"), ok("m⋅kg²/s⁻²"));
        }
        #[test]
        fn remove_redondant_spaces() {
            assert_eq!(
                format_unit("   m   .   kg     ^     -      2     /     s    ^   4   "),
                ok("m ⋅ kg⁻² / s⁴")
            );
        }
    }
}
