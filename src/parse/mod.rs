pub mod quantity;
pub mod range;
pub mod reference;

pub use reference::*;


pub trait RawRepr {
   fn raw(&self) -> &str;
}

use crate::math::DiagonalTensor;


pub type PermeabilityDiagonalTensor = DiagonalTensor<quantity::HydraulicPermeability>;
