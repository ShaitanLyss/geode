#![allow(unused)]

use clap::command;
use clap::Parser;
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



    //// Read the file content
    //let content = fs::read_to_string(file_path)?;
    //
    //// Print the content to the standard output
    ////println!("{}", content);
    //let parsed = dbg!(StrictYamlLoader::load_from_str(&content)?);
    //let doc = &parsed[0];
    //let mut out_str = String::new();
    //
    //let mut emitter = StrictYamlEmitter::new(&mut out_str);
    //dbg!(emitter.dump(doc).unwrap());
    //dbg!(out_str);

    Ok(())
}
