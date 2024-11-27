mod vec3;
pub use self::vec3::*;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DiagonalTensor<T> {
    xx: T,
    yy: T,
    zz: T
}
