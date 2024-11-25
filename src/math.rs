mod vec3;
pub use self::vec3::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagonalTensor<T> {
    xx: T,
    yy: T,
    zz: T
}
