#![allow(unused)]
pub mod model;
pub mod parse;
pub mod shape;
pub mod math;

pub use model::GeoscienceModel;


use std::error::Error as StdError;


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_compositional_injection_basic() {
        assert!(fs::read_to_string("example_configs/compositional-injection-basic.yaml").expect("example config file should be present")
            .parse::<GeoscienceModel>().is_ok());
    }
}
