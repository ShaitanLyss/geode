#![allow(unused)]
use super::error::ParseError;
use super::RawRepr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{de::Error, Deserialize, Serialize};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ParsedValue<L> {
    raw: String,
    parsed: L,
}

impl<L> RawRepr for ParsedValue<L> {
    fn raw(&self) -> &str {
        &self.raw
    }
}

impl<T> FromStr for ParsedValue<T>
where
    T: FromStr + DefaultUnit + Debug,
    T::Err: std::error::Error,
{
    type Err = ParseError<T>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ParsedValue::new(s)?)
    }
}

impl<L> Serialize for ParsedValue<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Only serialize the `raw` field as "value"
        serializer.serialize_str(&self.raw)
    }
}

impl<'de, L> Deserialize<'de> for ParsedValue<L>
where
    L: FromStr + Debug + DefaultUnit,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // First, deserialize the `raw` field as a string
        let raw: String = Deserialize::deserialize(deserializer)?;

        // Then attempt to parse it into L
        let parsed: L = raw
            .parse()
            .map_err(|_| D::Error::custom(format!("Failed to parse '{}'", raw)))?;

        // Return the new ParsedValue instance
        Ok(ParsedValue { raw, parsed })
    }
}

lazy_static! {
    static ref HAS_UNIT_RE: Regex = Regex::new(
        r"^(reference )?([+-]?[\d. _]+(?:e(?:\+|-)?[\d]+)?)[ \t]*([-°a-zA-Z][-+/\w]*)?$"
    )
    .unwrap();
}

impl<L> ParsedValue<L>
where
    L: FromStr + Debug + DefaultUnit,
    <L as FromStr>::Err: Debug,
{
    // Constructor to create a new ParsedValue
    pub fn new(raw: &str) -> Result<Self, ParseError<L>> {
        if let Some(captures) = HAS_UNIT_RE.captures(raw) {
            let _is_reference = captures.get(1).is_some();
            let prepped_raw = format!(
                "{} {}",
                captures[2].to_string().replace(" ", "").replace("_", ""),
                if let Some(unit) = captures.get(3) {
                    unit.as_str()
                } else {
                    L::DEFAULT_UNIT
                }
            );
            // Parse the string into L, or handle failure
            let parsed = prepped_raw
                .parse::<L>()
                .map_err(|e| ParseError::UnrecognizedQuantity(e))?;
            Ok(ParsedValue {
                raw: raw.to_string(),
                parsed,
            })
        } else {
            Err(ParseError::InvalidQuantityFormat(raw.to_string()))
        }
    }

    /// Getter for the parsed quantity
    pub fn parsed(&self) -> &L {
        &self.parsed
    }

    /// Raw representation of the quantity as written by the user
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

use uom::si::f64 as si;

pub trait DefaultUnit {
    const DEFAULT_UNIT: &str;
}

/// Length (default: kilometers, since distances in geoscience are often measured in km)
pub type Length = ParsedValue<si::Length>;

impl DefaultUnit for si::Length {
    const DEFAULT_UNIT: &str = "km";
}

/// Mass (default: grams, since small mass quantities in geoscience, especially in analysis, use grams)
pub type Mass = ParsedValue<si::Mass>;

impl DefaultUnit for si::Mass {
    const DEFAULT_UNIT: &str = "g";
}

/// Time (default: years, due to the typical timescales in geoscience, especially for geological processes)
pub type Time = ParsedValue<si::Time>;
impl DefaultUnit for si::Time {
    const DEFAULT_UNIT: &'static str = "yr"; // Years are commonly used in geoscience
}

/// Temperature (default: Celsius, as temperature is often measured in Celsius in geoscience contexts)
pub type Temperature = ParsedValue<si::ThermodynamicTemperature>;

impl DefaultUnit for si::ThermodynamicTemperature {
    const DEFAULT_UNIT: &'static str = "°C";
}

/// Pressure (default: pascal, as pressure is often measured in pascal in scientific contexts)
pub type Pressure = ParsedValue<si::Pressure>;

impl DefaultUnit for si::Pressure {
    const DEFAULT_UNIT: &'static str = "Pa";
}

/// Volume (default: cubic meters, which is the SI unit for volume)
pub type Volume = ParsedValue<si::Volume>;

impl DefaultUnit for si::Volume {
    const DEFAULT_UNIT: &'static str = "m³";
}

/// Molar Mass (default: grams per mole, as it's commonly used in geoscience)
pub type MolarMass = ParsedValue<si::MolarMass>;

impl DefaultUnit for si::MolarMass {
    const DEFAULT_UNIT: &'static str = "g/mol";
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{DefaultUnit, ParsedValue};
    use std::fmt::Debug;
    use uom::si::{f64::Length, length::*};

    fn make_parsed<L>(raw: &str, parsed: L) -> ParsedValue<L>
    where
        L: FromStr + Debug + DefaultUnit,
    {
        ParsedValue {
            raw: raw.to_string(),
            parsed,
        }
    }

    #[test]
    fn parse_length_with_valid_input() {
        fn make_length(raw: &str) -> super::Length {
            super::Length::new(raw).unwrap()
        }
        assert_eq!(
            make_length("10 m"),
            make_parsed("10 m", Length::new::<meter>(10.))
        );
        assert_eq!(
            make_length("10m"),
            make_parsed("10m", Length::new::<meter>(10.))
        );
        assert_eq!(
            make_length("10     m"),
            make_parsed("10     m", Length::new::<meter>(10.))
        );
        assert_eq!(
            make_length("10"),
            make_parsed("10", Length::new::<kilometer>(10.))
        );
        assert_eq!(
            make_length("100 000 m"),
            make_parsed("100 000 m", Length::new::<kilometer>(100.))
        );
        assert_eq!(
            make_length("1 meter"),
            make_parsed("1 meter", Length::new::<meter>(1.))
        );
        assert_eq!(
            make_length("2 meters"),
            make_parsed("2 meters", Length::new::<meter>(2.))
        );
        assert_eq!(
            make_length("-1"),
            make_parsed("-1", Length::new::<kilometer>(-1.))
        )
    }
    #[test]
    fn parse_length_with_invalid_input() {
        fn attempt_length_parse(raw: &str) {
            let result = super::Length::new(raw);
            assert!(result.is_err(), "Expected error for input '{}'", raw);
        }

        attempt_length_parse("ten m"); // Invalid number format
        attempt_length_parse("10 xyz"); // Unrecognized unit
        attempt_length_parse(""); // Empty string
                                  //attempt_length_parse("10 10 m"); // Invalid format
        attempt_length_parse("reference m"); // Missing number
    }
}
