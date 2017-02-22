//! Defines the metric mass standards as newtypes

use std::{self, fmt};

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Kilogram(pub f64);
pub type Kilograms = Kilogram;

impl_basic_ops!(Kilogram);
impl_div_same!(Kilogram);
impl_scalar_ops!(Kilogram);
impl_unit_debug!(Kilogram => "{}kg");
impl_partial_ord!(Kilogram);
