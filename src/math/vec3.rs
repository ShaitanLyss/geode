use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::parse::quantity::Length;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct Vec3 {
    x: Length,
    y: Length,
    z: Length
}
