use serde::{Serialize, Deserialize};
use crate::parse::quantity::Length;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Vec3 {
    x: Length,
    y: Length,
    z: Length
}
