//! Defines the imperial mass standards as newtypes

use core::{self, fmt};

use composite::UnitName;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Ton(pub f64);
pub type Tons = Ton;

impl_full_unit!(Ton);
impl_unit_debug!(Ton => "(imperial ton)");

impl_from_cf!(Ton <===>  2000.0 Pound);
impl_from_cf!(Ton <===> 32000.0 Ounce);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Pound(pub f64);
pub type Pounds = Pound;

impl_full_unit!(Pound);
impl_unit_debug!(Pound => "lb");

impl_from_cf!(Pound <===> 16.0 Ounce);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Ounce(pub f64);
pub type Ounces = Ounce;

impl_full_unit!(Ounce);
impl_unit_debug!(Ounce => "oz");
