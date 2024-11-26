#![allow(unused)]

use clap::Parser;
use clap::Subcommand;
use clap::command;
use geode::GeoscienceModel;
use std::error::Error;
use strict_yaml_rust::StrictYamlEmitter;
use strict_yaml_rust::StrictYamlLoader;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// A human first geoscience configuration format, for generating input decks for various simulation
/// frameworks.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///// Geode config file to read
    //#[arg(short, long)]
    //config_file: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate,
    Run,
    Parse { config_file: PathBuf },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    //if let Some(command) = args.command {
    match args.command {
        Commands::Generate => todo!(),
        Commands::Run => todo!(),
        Commands::Parse { config_file } => {
            let model: GeoscienceModel = fs::read_to_string(config_file)?.parse()?;
            println!("{:#?}", model);
        }
    }
    //}

    Ok(())
}
