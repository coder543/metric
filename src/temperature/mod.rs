//! This module provides newtypes for performant and safe unit-aware calculations with temperatures.

pub mod fahrenheit;
pub mod celsius;
pub mod kelvin;

pub use fahrenheit::Fahrenheit;
pub use celsius::Celsius;
pub use kelvin::Kelvin;
