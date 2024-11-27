pub mod quantity;
pub mod range;
pub mod reference;
pub mod unit;

pub use reference::*;
pub use quantity::{QUANTITY_PATTERN, NO_REF_QUANTITY_PATTERN, RANGE_PATTERN, QUANTITY_RE};


pub trait RawRepr {
   fn raw(&self) -> &str;
}

use crate::math::DiagonalTensor;


pub type PermeabilityDiagonalTensor = DiagonalTensor<quantity::HydraulicPermeability>;
