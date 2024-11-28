#![allow(unused)]
use geode::GeoscienceModel;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(GeoscienceModel);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
