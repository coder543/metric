//! Defines various constants with meaningful units

use mass::metric::{Kilogram, Kilograms};
use length::astronomical::{AU, AUs};
use length::metric::{Kilometer, Kilometers};

#[derive(Debug)]
pub struct PlanetInfo {
    pub name: &'static str,
    pub mass: Kilograms,
    pub dist_from_sun: AUs,
    pub radius: Kilometers,
}

pub const EARTH: PlanetInfo = PlanetInfo {
    name: "Earth",
    mass: Kilogram(5.9722e24),
    dist_from_sun: AU(1.0),
    radius: Kilometer(6371.0)
};