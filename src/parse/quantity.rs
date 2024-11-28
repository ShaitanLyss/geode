#![allow(unused)]
use super::unit::{format_unit, FormatUnitError};
use super::RawRepr;
use crate::StdError;
use const_format::{concatcp, formatcp};
use lazy_static::lazy_static;
use regex::Regex;
use schemars::schema::{
    InstanceType, Schema, SchemaObject, SingleOrVec, StringValidation, SubschemaValidation,
};
use schemars::JsonSchema;
use serde::{de::Error, Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use thiserror::Error;
use uom::str::ParseQuantityError as UomParseError;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Quantity<L> {
    raw: String,
    parsed: L,
}

impl<T> JsonSchema for Quantity<T> {
    fn schema_name() -> String {
        String::from("Quantity")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema = SchemaObject::default();
        //schema.instance_type = Some(SingleOrVec::Single(Box::new(InstanceType::String)));
        schema.subschemas = Some(Box::new(SubschemaValidation {
            one_of: Some(vec![
                // Schema for string type
                Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                    string: Some(Box::new(StringValidation {
                        pattern: Some(NO_REF_QUANTITY_PATTERN.to_string()),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                // Schema for number type
                Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Number))),
                    ..Default::default()
                }),
            ]),
            ..Default::default()
        }));

        Schema::Object(schema)
    }
}

impl<T> Display for Quantity<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl<L> RawRepr for Quantity<L> {
    fn raw(&self) -> &str {
        &self.raw
    }
}

#[derive(Debug, Error)]
pub enum ParseQuantityError {
    #[error("invalid quantity format : '{0}', should be 'value [unit]'")]
    InvalidFormat(String),
    #[error("this quantity can't be a reference, please remove the 'ref' or 'reference' keyword")]
    NoReference,
    #[error("invalid unit format: {0}")]
    InvalidUnitFormat(#[from] FormatUnitError),
    #[error("quantity not recognized: '{0}'")]
    Unrecognized(#[from] UomParseError),
}

impl<T> FromStr for Quantity<T>
where
    T: FromStr<Err = UomParseError> + DefaultUnit + Debug,
{
    type Err = ParseQuantityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Quantity::new(s)?)
    }
}

impl<T> Serialize for Quantity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Only serialize the `raw` field as "value"
        serializer.serialize_str(&self.raw)
    }
}

impl<'de, T> Deserialize<'de> for Quantity<T>
where
    T: FromStr<Err = UomParseError> + Debug + DefaultUnit,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // First, deserialize the `raw` field as a string
        let raw: &str = Deserialize::deserialize(deserializer)?;
        Ok(Quantity::new(raw).map_err(|e| D::Error::custom(e))?)
    }
}

const PARTIAL_QUANTITY_PATTERN: &str =
    r"\s*([+-]?[\d_ ]*?\.?[\d_ ]+?(?:e(?:\+|-)?[.\d]+)?)[ \t]*([^\d\s.](?:.*?[^.])?)?\s*";

pub const NO_REF_QUANTITY_PATTERN: &str = formatcp!("^{PARTIAL_QUANTITY_PATTERN}$");

const PARTIAL_REFERENCE_PATTERN: &str = concatcp!(r"\s*(reference|ref)?", PARTIAL_QUANTITY_PATTERN);
pub const QUANTITY_PATTERN: &str = formatcp!("^{PARTIAL_REFERENCE_PATTERN}$");

pub const RANGE_PATTERN: &str =
    formatcp!(r"^{PARTIAL_QUANTITY_PATTERN}\s*..\s*{PARTIAL_QUANTITY_PATTERN}$");

lazy_static! {
    pub static ref QUANTITY_RE: Regex = Regex::new(QUANTITY_PATTERN).unwrap();
}

pub fn get_unit(quantity: &str) -> Option<&str> {
    Some(QUANTITY_RE.captures(quantity)?.get(3)?.as_str())
}

impl<T> Quantity<T>
where
    T: FromStr<Err = UomParseError> + Debug + DefaultUnit,
{
    // Constructor to create a new ParsedValue
    pub fn new(raw: &str) -> Result<Self, ParseQuantityError> {
        QUANTITY_RE.to_string();
        if let Some(captures) = QUANTITY_RE.captures(raw) {
            if captures.get(1).is_some() {
                return Err(ParseQuantityError::NoReference);
            }
            let mut unit: String = T::DEFAULT_UNIT.to_string();
            if let Some(u) = captures.get(3) {
                unit = format_unit(u.as_str())?;
            }

            let value = &captures[2];
            let mut pretty_value = String::with_capacity(value.len());
            let mut prepped_value = String::with_capacity(value.len());

            for c in value.chars() {
                match c {
                    ' ' => pretty_value.push(' '),
                    '_' => pretty_value.push(' '),
                    _ => {
                        pretty_value.push(c);
                        prepped_value.push(c);
                    }
                }
            }

            let prepped_raw = format!("{} {}", prepped_value, &unit);

            Ok(Quantity {
                parsed: prepped_raw.parse()?,
                raw: format!(
                    "{}{}{}",
                    pretty_value,
                    if unit.len() > 0 { " " } else { "" },
                    &unit
                ),
            })
        } else {
            Err(ParseQuantityError::InvalidFormat(raw.to_string()))
        }
    }

    /// Getter for the parsed quantity
    pub fn parsed(&self) -> &T {
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

/// Ratio (unit less value resulting from calculating the ratio of two quantities)
pub type Ratio = Quantity<si::Ratio>;

impl DefaultUnit for si::Ratio {
    const DEFAULT_UNIT: &str = "";
}

/// Area (default: km²)
pub type Area = Quantity<si::Area>;

impl DefaultUnit for si::Area {
    const DEFAULT_UNIT: &str = "km²";
}

/// Compressibility (default: Pa⁻¹)
pub type Compressibility = Quantity<si::Compressibility>;

impl DefaultUnit for si::Compressibility {
    const DEFAULT_UNIT: &str = "Pa⁻¹";
}

/// HydraulicPermeability (default: darcy)
pub type HydraulicPermeability = Quantity<si::HydraulicPermeability>;

impl DefaultUnit for si::HydraulicPermeability {
    const DEFAULT_UNIT: &str = "mD";
}

/// Length (default: kilometers, since distances in geoscience are often measured in km)
pub type Length = Quantity<si::Length>;

impl DefaultUnit for si::Length {
    const DEFAULT_UNIT: &str = "km";
}

/// Mass (default: grams, since small mass quantities in geoscience, especially in analysis, use grams)
pub type Mass = Quantity<si::Mass>;

impl DefaultUnit for si::Mass {
    const DEFAULT_UNIT: &str = "g";
}

/// Time (default: years, due to the typical timescales in geoscience, especially for geological processes)
pub type Time = Quantity<si::Time>;
impl DefaultUnit for si::Time {
    const DEFAULT_UNIT: &'static str = "yr"; // Years are commonly used in geoscience
}

/// Temperature (default: Celsius, as temperature is often measured in Celsius in geoscience contexts)
pub type Temperature = Quantity<si::ThermodynamicTemperature>;

impl DefaultUnit for si::ThermodynamicTemperature {
    const DEFAULT_UNIT: &'static str = "°C";
}

/// Pressure (default: pascal, as pressure is often measured in pascal in scientific contexts)
pub type Pressure = Quantity<si::Pressure>;

impl DefaultUnit for si::Pressure {
    const DEFAULT_UNIT: &'static str = "Pa";
}

/// Volume (default: cubic meters, which is the SI unit for volume)
pub type Volume = Quantity<si::Volume>;

impl DefaultUnit for si::Volume {
    const DEFAULT_UNIT: &'static str = "m³";
}

/// Molar Mass (default: grams per mole, as it's commonly used in geoscience)
pub type MolarMass = Quantity<si::MolarMass>;

impl DefaultUnit for si::MolarMass {
    const DEFAULT_UNIT: &'static str = "g/mol";
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{DefaultUnit, Pressure, Quantity};
    use std::fmt::Debug;
    use uom::si::{f64::Length, length::*};

    fn make_parsed<L>(raw: &str, parsed: L) -> Quantity<L>
    where
        L: FromStr + Debug + DefaultUnit,
    {
        Quantity {
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
            make_parsed("10 m", Length::new::<meter>(10.))
        );
        assert_eq!(
            make_length("10     m"),
            make_parsed("10 m", Length::new::<meter>(10.))
        );
        assert_eq!(
            make_length("10"),
            make_parsed("10 km", Length::new::<kilometer>(10.))
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
            make_parsed("-1 km", Length::new::<kilometer>(-1.))
        );
        assert_eq!(
            super::Compressibility::new("1e-09 Pa-1").expect("Valid quantity should be parsed."),
            Quantity {
                parsed: uom::si::f64::Compressibility::new::<uom::si::compressibility::pascal>(
                    1e-09
                ),
                raw: "1e-09 Pa⁻¹".to_string()
            }
        );
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

    #[test]
    fn parse_reference_should_err() {
        let raw = "reference 5 000 000";
        assert!(raw.parse::<Pressure>().is_err());
    }
}
