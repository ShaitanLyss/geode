#![allow(unused)]
use geode::{parse::{quantity::{self, Mass}, range::Range}, shape::Cuboid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    cuboid: Cuboid
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let length_value = quantity::Length::new("1 000 m")?;
    dbg!(&length_value);

    dbg!("0 .. 4".parse::<Range<f64>>()?);

    dbg!("1 kg .. -5 g".parse::<Range<Mass>>()?);

    let test: Test = dbg!(serde_yaml::from_str("cuboid:\n x: 0..1\n y: 1..2\n z: 0..-1")?);

    //// Print the parsed value to ensure it's working correctly
    //println!("Parsed value: {:?}", length_value.parsed());
    //
    //// Serialize the Test struct containing the Length value
    //let test_instance = Test { x: length_value };
    //
    //// Serialize Test to a JSON string (for example)
    //let serialized = serde_yaml::to_string(&test_instance).unwrap();
    //println!("Serialized Test:\n{}", serialized);

    // Deserialize Test from a JSON string
    //

    //let deserialized: Test = serde_yaml::from_str("x: '0.4 m'").unwrap();
    //println!("Deserialized Test: {:?}", deserialized);
    Ok(())
}
