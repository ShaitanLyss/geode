extern crate strict_yaml_rust;
extern crate uom;

mod model;
mod parse;

use strict_yaml_rust::StrictYamlLoader;
use std::error::Error;

use std::env;
use std::fs;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided
    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: {} <file_path>", args[0])?;
        std::process::exit(1);
    }

    // Get the file path from the arguments
    let file_path = &args[1];

    // Read the file content
    let content = fs::read_to_string(file_path)?;

    // Print the content to the standard output
    //println!("{}", content);
    dbg!(StrictYamlLoader::load_from_str(&content)?);

    Ok(())
}
