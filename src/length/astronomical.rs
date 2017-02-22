//! Defines the astronomical length standards as newtypes

use std::{self, fmt};

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct AU(pub f64);
pub type AUs = AU;

impl_basic_ops!(AU);
impl_div_same!(AU);
impl_scalar_ops!(AU);
impl_unit_debug!(AU => "{}AU");
impl_partial_ord!(AU);