//! Defines various constants with meaningful units

use mass::metric::Kilograms;

#[derive(Debug)]
pub struct PlanetInfo {
    pub name: &'static str,
    pub mass: Kilograms
}