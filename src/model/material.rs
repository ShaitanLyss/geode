use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::parse::{quantity::{Compressibility, HydraulicPermeability, Pressure, Ratio}, PermeabilityDiagonalTensor, WithReference};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct Material {
    pub pressure: Option<WithReference<Pressure>>,
    pub porosity: Option<WithReference<Ratio>>,
    pub permeability: Option<PermeabilityDiagonalTensor>,
    pub compressibility: Option<Compressibility>
}
