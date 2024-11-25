use crate::shape::Cuboid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Domain {
    cuboid: Cuboid,
    materials: Vec<String>
}
