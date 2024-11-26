use geode::GeoscienceModel;
use std::fs;

fn parse_compositional_injection_basic() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{:#?}",
        fs::read_to_string("example_configs/compositional-injection-basic.yaml")?
            .parse::<GeoscienceModel>()?
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    parse_compositional_injection_basic()?;
    Ok(())
}

