#![allow(unused)]
use geode::{parse::{quantity::{self, Mass, Pressure}, range::Range, WithReference}, shape::Cuboid};
use itertools::Itertools;
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
    dbg!("5 000 000".parse::<Pressure>()?);

    dbg!("z".parse::<WithReference<char>>());
    dbg!("m.k*s".split(".").collect_vec());
    Ok(())
}
