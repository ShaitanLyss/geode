use strict_yaml_rust::StrictYamlLoader;
use std::fs;
fn main() {

    print!("{:?}", StrictYamlLoader::load_from_str(&fs::read_to_string("./frameworks/geos.geoffrey.yaml").unwrap()));

    
}
