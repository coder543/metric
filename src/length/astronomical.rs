//! Defines the astronomical length standards as newtypes

use std::{self, fmt};

use metric::*;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct AU(pub f64);
pub type AUs = AU;

impl_basic_ops!(AU);
impl_div_same!(AU);
impl_scalar_ops!(AU);
impl_unit_debug!(AU => "{}m");
impl_partial_ord!(AU);

const M_PER_AU: f64 = 149_597_870_700.;

impl_from!(Kilometer  => AU,   |km|   km /  (M_PER_AU / 1000.));
impl_from!(Meter      => AU,    |m|    m /  (M_PER_AU *    1.));
impl_from!(Centimeter => AU,   |cm|   cm /  (M_PER_AU *  100.));
impl_from!(Millimeter => AU,   |mm|   mm /  (M_PER_AU * 1000.));