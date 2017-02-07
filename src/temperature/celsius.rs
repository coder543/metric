//! Defines the `Celsius` temperature newtype and related trait impls

use std;

use fahrenheit::Fahrenheit;
use kelvin::Kelvin;

/// A newtype that wraps around `f64` and provides convenience functions for unit-safe and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Celsius(pub f64);

impl_basic_ops!(Celsius);
impl_unit_debug!(Celsius => "{}°C");
impl_partial_ord!(Celsius);

impl_from!(Fahrenheit => Celsius, |f| (f -  32.) * 5. / 9.);
impl_from!(Kelvin     => Celsius, |k|  k - 273.);
