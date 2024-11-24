#![allow(unused)]
//! The `model` module contains the core definitions for geoscience physics modeling.
//!
//! - `Physics`: Represents different types of physical models.
//! - `CompositionalPhysics`: A specific model for compositional systems.

use compositional::CompositionalPhysics;

mod compositional {
    use crate::parse::quantity::*;
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    /// Represents a component in a compositional physics model, such as a
    /// molecule or chemical species.
    #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
    pub struct Component {
        pub name: String,
        pub mass: MolarMass,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CompositionalPhysics {
        pub components: HashMap<String, Component>,
        pub phases: Vec<String>,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Physics {
    Compositional(CompositionalPhysics),
}

pub struct GeoscienceModel {
    pub physics: Vec<Physics>,
}
