#![allow(unused)]

use clap::command;
use clap::Parser;
use geode::GeoscienceModel;
use strict_yaml_rust::StrictYamlEmitter;
use strict_yaml_rust::StrictYamlLoader;
use std::error::Error;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;


/// A human first geoscience configuration format, for generating input decks for various simulation
/// frameworks.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Geode config file to read
    #[arg(short, long)]
    config_file: PathBuf
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    dbg!(args.config_file.to_str());
    let model: GeoscienceModel = fs::read_to_string(args.config_file)?.parse()?;
    println!("{:?}", model);

    Ok(())
}
