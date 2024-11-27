#![allow(unused)]
//! The `model` module contains the core definitions for geoscience physics modeling.
//!
//! - `Physics`: Represents different types of physical models.
//! - `CompositionalPhysics`: A specific model for compositional systems.

pub mod domain;
pub mod material;
pub use material::Material;
use schemars::JsonSchema;

use std::{collections::HashMap, str::FromStr};

use self::domain::Domain;
use compositional::CompositionalPhysics;
use serde::{Deserialize, Serialize};

mod compositional {
    use crate::parse::quantity::*;
    use std::collections::HashMap;

    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Represents a component in a compositional physics model, such as a
    /// molecule or chemical species.
    #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, JsonSchema)]
    pub struct Component {
        pub mass: MolarMass,
    }

    /// # Compositional Physics
    ///
    /// Describe compositional physics YEAH
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
    pub struct CompositionalPhysics {
        pub components: HashMap<String, Component>,
        pub phases: Vec<String>,
    }
}

/// Hello Lyss, you're such a bg.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct Physics {
    compositional: CompositionalPhysics,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct GeoscienceModel {
    pub physics: Physics,
    pub domain: Domain,
    #[serde(default)]
    pub materials: HashMap<String, Option<Material>>,
}

impl FromStr for GeoscienceModel {
    type Err = serde_yaml::Error;

    /// Converts a YAML formatted string into a `GeoscienceModel`.
    /// This allows the use of the `FromStr` trait to parse YAML strings
    /// directly into the model, with graceful error handling.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_yaml(s)
    }
}

impl GeoscienceModel {
    /// Constructs a `GeoscienceModel` from a YAML string input using Serde.
    /// It attempts to deserialize the input and returns a Result with either
    /// a `GeoscienceModel` instance on success, or a `serde_yaml::Error` on failure.
    fn from_yaml(s: &str) -> Result<Self, serde_yaml::Error> {
        Ok(serde_yaml::from_str(s)?)
    }
}
