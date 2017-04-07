//! Defines the astronomical length standards as newtypes

use core::{self, fmt};

use length::imperial::*;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct AU(pub f64);
pub type AUs = AU;

impl_basic_ops!(AU);
impl_div_same!(AU);
impl_scalar_ops!(AU);
impl_unit_debug!(AU => "{}AU");
impl_partial_ord!(AU);

impl_from_cf!(AU <===>      92955807.27302553 Mile);
impl_from_cf!(AU <===>  163602220800.52493    Yard);
impl_from_cf!(AU <===>  490806662401.5748     Foot);
impl_from_cf!(AU <===> 5889679948818.898      Inch);