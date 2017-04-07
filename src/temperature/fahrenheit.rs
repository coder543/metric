//! Defines the `Fahrenheit` temperature newtype and related trait impls

use core::{self, fmt};

use composite::UnitName;

use temperature::Celsius;
use temperature::Kelvin;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Fahrenheit(pub f64);

impl_basic_ops!(Fahrenheit);
impl_unit_debug!(Fahrenheit => "°F");
impl_partial_ord!(Fahrenheit);

impl_from!(Celsius => Fahrenheit, |c| c * 9. / 5. + 32.);
impl_from!(Kelvin  => Fahrenheit, |k| k * 9. / 5. - 459.67);
