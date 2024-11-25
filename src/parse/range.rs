#![allow(unused)]
use std::str::FromStr;
use thiserror::Error;

use serde::{de::Error, Deserialize, Serialize};

use super::RawRepr;

#[derive(Clone, Debug, PartialEq)]
pub struct Range<T> {
    start: T,
    end: T,
    raw: String,
}

impl<T> Serialize for Range<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.raw)
    }
}

impl<'de, T> Deserialize<'de> for Range<T>
where
    T: FromStr, T::Err: std::error::Error+ 'static
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw: String = Deserialize::deserialize(deserializer)?;
        let range: Range<T> = raw.parse().map_err(|e| D::Error::custom(e))?;

        Ok(range)
    }
}

impl<T> RawRepr for Range<T> {
    /// Returns a raw string representation as written by the user.
    fn raw(&self) -> &str {
        &self.raw
    }
}

use std::fmt::{Debug, Display};
#[derive(Debug, Error, PartialEq)]
pub enum ParseRangeError<T: FromStr> where T::Err: std::error::Error + 'static  {
    #[error("Wrong number of elements for a range, it should be 2. Example: 0 .. 1")]
    WrongArgNumber,
    #[error("Failed to parse start bound: {0}")]
    InvalidStart(#[source] T::Err),
    #[error("Failed to parse end bound: {0}")]
    InvalidEnd(#[source] T::Err),
}

impl<T> FromStr for Range<T>
where
    T: FromStr, T::Err: std::error::Error + 'static
{
    type Err = ParseRangeError<T>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("..").map(|part| part.trim()).collect();

        if parts.len() != 2 {
            return Err(ParseRangeError::WrongArgNumber);
        }

        let start: T = parts[0]
            .parse()
            .map_err(|e| ParseRangeError::InvalidStart(e))?;
        let end: T = parts[1].parse().map_err(|e| ParseRangeError::InvalidEnd(e))?;

        Ok(Range {
            start,
            end,
            raw: s.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn parse_valid_range() {
        let raw = "0 .. 1";
        let range: Range<i32> = raw.parse().expect("Valid range should be parsed");
        assert_eq!(
            range,
            Range {
                start: 0,
                end: 1,
                raw: raw.to_string()
            }
        )
    }
    #[test]
    fn serialize_range() {
        let range = Range {
            start: 0,
            end: 1,
            raw: "0 .. 1".to_string(),
        };
        let serialized = serde_json::to_string(&range).expect("Serialization should succeed");
        assert_eq!(serialized, "\"0 .. 1\"");
    }
    //
    //#[test]
    //fn deserialize_range() {
    //    let raw = "\"0 .. 1\"";
    //    let range: Range<i32> = serde_json::from_str(raw).expect("Deserialization should succeed");
    //    assert_eq!(
    //        range,
    //        Range {
    //            start: 0,
    //            end: 1,
    //            raw: "0 .. 1".to_string()
    //        }
    //    );
    //}

    #[test]
    fn parse_invalid_range_wrong_arg_number() {
        let raw = "0 .. 1 .. 2";
        let result: Result<Range<i32>, _> = raw.parse();
        assert_eq!(result.unwrap_err(), ParseRangeError::WrongArgNumber);
    }

    #[test]
    fn parse_invalid_range_invalid_start() {
        let raw = "a .. 1";
        let result: Result<Range<i32>, _> = raw.parse();
        assert_eq!(result.unwrap_err(), ParseRangeError::InvalidStart("a".parse::<i32>().unwrap_err()));
    }

    #[test]
    fn parse_invalid_range_invalid_end() {
        let raw = "0 .. b";
        let result: Result<Range<i32>, _> = raw.parse();
        assert_eq!(result.unwrap_err(), ParseRangeError::InvalidEnd("b".parse::<i32>().unwrap_err()));
    }
}