//! Defines the `Kelvin` temperature newtype and related trait impls

use std;

use fahrenheit::Fahrenheit;
use celsius::Celsius;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Kelvin(pub f64);

impl_basic_ops!(Kelvin);
impl_unit_debug!(Kelvin => "{}K");
impl_partial_ord!(Kelvin);

impl_from!(Celsius    => Kelvin, |c|  c + 273.);
impl_from!(Fahrenheit => Kelvin, |f| (f + 459.67) * 5. / 9.);
