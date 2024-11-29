use strict_yaml_rust::StrictYamlLoader;
use std::fs;

fn main() {
    println!("{:?}\n", StrictYamlLoader::load_from_str(&fs::read_to_string("./frameworks/geos.geoffrey.yaml").unwrap()));
    geoffrey::geoffrey();
}
