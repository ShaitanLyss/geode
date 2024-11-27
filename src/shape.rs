use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::parse::{quantity::Length, range::Range};
use crate::math::Vec3;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct Cuboid {
    x: Range<Length>,
    y: Range<Length>,
    z: Range<Length>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct Sphere {
    center: Vec3, 
    radius: Length 
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn cuboid_serde_yaml() {
        let cuboid = Cuboid {
            x: Range::from_str("0..10").expect("Valid range should be parsed"),
            y: Range::from_str("2..20").expect("Valid range should be parsed"),
            z: Range::from_str("3..30").expect("Valid range should be parsed"),
        };
        
        let yaml_string = serde_yaml::to_string(&cuboid).expect("Failed to serialize to YAML");
        let deserialized_cuboid: Cuboid = serde_yaml::from_str(&yaml_string).expect("Failed to deserialize from YAML");
        
        assert_eq!(cuboid, deserialized_cuboid);
    }
    
    #[test]
    fn cuboid_serde_json() {
        let cuboid = Cuboid {
            x: Range::from_str("1..10").expect("Valid range should be parsed"),
            y: Range::from_str("2..20").expect("Valid range should be parsed"),
            z: Range::from_str("3..30").expect("Valid range should be parsed"),
        };
        
        let json_string = serde_json::to_string(&cuboid).expect("Failed to serialize to JSON");
        let deserialized_cuboid: Cuboid = serde_json::from_str(&json_string).expect("Failed to deserialize from JSON");
        
        assert_eq!(cuboid, deserialized_cuboid);
    }
}
