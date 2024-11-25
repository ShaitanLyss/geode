use serde::{Deserialize, Serialize};

use crate::parse::{quantity::{HydraulicPermeability, Pressure, Ratio}, PermeabilityDiagonalTensor, WithReference};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Material {
    pressure: Option<WithReference<Pressure>>,
    porosity: Option<WithReference<Ratio>>,
    permeability: Option<PermeabilityDiagonalTensor>
}