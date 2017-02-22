//! This module provides newtypes for performant and safe unit-aware calculations with temperatures.

pub mod celsius;
pub mod kelvin;
pub mod fahrenheit;

pub use self::celsius::Celsius;
pub use self::kelvin::Kelvin;
pub use self::fahrenheit::Fahrenheit;
