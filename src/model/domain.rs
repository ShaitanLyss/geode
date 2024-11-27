use crate::shape::Cuboid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct Domain {
    cuboid: Cuboid,
    materials: Vec<String>
}
