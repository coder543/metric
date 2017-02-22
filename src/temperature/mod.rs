//! This module provides newtypes for performant and safe unit-aware calculations with temperatures.

pub mod fahrenheit;
pub mod celsius;
pub mod kelvin;

pub use self::fahrenheit::Fahrenheit;
pub use self::celsius::Celsius;
pub use self::kelvin::Kelvin;
