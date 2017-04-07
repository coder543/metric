//! Defines the imperial mass standards as newtypes

use core::{self, fmt};

use composite::UnitName;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Ton(pub f64);
pub type Tons = Ton;

impl_basic_ops!(Ton);
impl_div_same!(Ton);
impl_scalar_ops!(Ton);
impl_unit_debug!(Ton => "(imperial ton)");
impl_partial_ord!(Ton);

impl_from_cf!(Ton <===>  2000.0 Pound);
impl_from_cf!(Ton <===> 32000.0 Ounce);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Pound(pub f64);
pub type Pounds = Pound;

impl_basic_ops!(Pound);
impl_div_same!(Pound);
impl_scalar_ops!(Pound);
impl_unit_debug!(Pound => "lb");
impl_partial_ord!(Pound);

impl_from_cf!(Pound <===> 16.0 Ounce);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Ounce(pub f64);
pub type Ounces = Ounce;

impl_basic_ops!(Ounce);
impl_div_same!(Ounce);
impl_scalar_ops!(Ounce);
impl_unit_debug!(Ounce => "oz");
impl_partial_ord!(Ounce);