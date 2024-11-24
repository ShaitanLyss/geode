use std::{fmt::{Debug, Display}, str::FromStr};


#[derive(Debug)]
pub enum ParseError<T> where T: FromStr, <T as FromStr>::Err: Debug {
    InvalidQuantityFormat(String),
    UnrecognizedQuantity(<T as FromStr>::Err)
}

use ParseError::*;

impl<T> Display for ParseError<T> where T: FromStr, <T as FromStr>::Err: Debug  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InvalidQuantityFormat(s) => format!("Invalid Quantity Format : {}", s),
                UnrecognizedQuantity(s) => format!("Unrecognized quantity : '{s:?}'. Check the unit and value.")
            }
        )
    }
}

impl<T> std::error::Error for ParseError<T> where  T: FromStr+Debug, <T as FromStr>::Err: Debug {
}
