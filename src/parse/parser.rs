extern crate regex;

use std::str::FromStr;

use lazy_static::lazy_static;

use regex::Regex;
use serde::{Deserialize, Serialize};
use uom::{si::Quantity, Kind};

lazy_static! {
    /// Compiled regex for parsing a physic scalar value with optionally an unit and a reference
    /// keyword.
    static ref QUANTITY_REGEX: Regex = Regex::new(r"(reference)?\s*(.+)").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct ParsedValue<L> where L: FromStr {
    raw: String,
    #[serde(skip)]
    parsed: L
}

pub type Length = ParsedValue<uom::si::f64::Length>;

//use uom::si::{Dimension, Quantity};

//#[derive(Debug)]
//pub struct ParsedQuantity<D, U, V> 
//where
//    D: Dimension + ?Sized,
//    U: uom::si::Units<V> + ?Sized,
//    V: uom::Conversion<V> + uom::num_traits::Num,
//
//    {
//    pub is_reference: bool,
//    pub quantity: Quantity<D, U, V> 
//}
//
//
//pub fn parse_value<Q>(s: &str) -> Result<Q, &str>
//where
//    Q: FromStr + Kind
//{
//    if let Some(captures) = QUANTITY_REGEX.captures(s) {
//        let is_reference = captures.get(1).is_some();
//        let quantity = captures
//            .get(2)
//            .expect("Invalid format for physic quantity")
//            .as_str();
//
//        match Quantity::from_str(quantity) {
//            Ok(quantity) => Ok(quantity),
//            Err(_e) => Err("Failed to parse quantity")
//        }
//    } else {
//        Err("Invalid quantity string")
//    }
//}
//
//#[derive(Debug)]
//pub struct ParsedQuantity<Q>
//where
//    Q: Into<Quantity<D, U, V>> + ?Sized,
//{
//    pub is_reference: bool,
//    pub quantity: Q,
//}
//
