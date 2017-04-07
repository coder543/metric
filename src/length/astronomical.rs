//! Defines the astronomical length standards as newtypes

use core::{self, fmt};

use length::imperial::*;
use length::metric::*;

use composite::UnitName;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct AU(pub f64);
pub type AUs = AU;

impl_full_unit!(AU);
impl_unit_debug!(AU => "AU");

impl_from_cf!(AU <===>      92955807.27302553 Mile);
impl_from_cf!(AU <===>  163602220800.52493    Yard);
impl_from_cf!(AU <===>  490806662401.5748     Foot);
impl_from_cf!(AU <===> 5889679948818.898      Inch);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Lightyear(pub f64);
pub type Lightyears = Lightyear;

impl_full_unit!(Lightyear);
impl_unit_debug!(Lightyear => "ly");

impl_from_cf!(Lightyear <===>                        63241.07         AU);
impl_from_cf!(Lightyear <===>                9460729412789.648 Kilometer);
impl_from_cf!(Lightyear <===>             9460729412789648.0       Meter);
impl_from_cf!(Lightyear <===>           946072941278964800.0  Centimeter);
impl_from_cf!(Lightyear <===>          9460729412789648000.0  Millimeter);
impl_from_cf!(Lightyear <===>       9460729412789648000000.0  Micrometer);
impl_from_cf!(Lightyear <===>    9460729412789648000000000.0   Picometer);
impl_from_cf!(Lightyear <===> 9460729412789648000000000000.0  Femtometer);
impl_from_cf!(Lightyear <===>                5878624714659.916      Mile);
impl_from_cf!(Lightyear <===>            10346379497801454.0        Yard);
impl_from_cf!(Lightyear <===>            31039138493404360.0        Foot);
impl_from_cf!(Lightyear <===>           372469661920852352.0        Inch);