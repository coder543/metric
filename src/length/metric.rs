//! Defines the metric length standards as newtypes

use std::{self, fmt};

use imperial::*;
use astronomical::*;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Meter(pub f64);
pub type Meters = Meter;

impl_basic_ops!(Meter);
impl_div_same!(Meter);
impl_scalar_ops!(Meter);
impl_unit_debug!(Meter => "{}m");
impl_partial_ord!(Meter);

impl_from_cf!(AU        <===> 149597870700.0 Meter     );
impl_from_cf!(Kilometer <===>         1000.0 Meter     );
impl_from_cf!(Mile      <===>        1609.34 Meter     );
impl_from_cf!(Meter     <===>          100.0 Centimeter);
impl_from_cf!(Meter     <===>         1000.0 Millimeter);
impl_from_cf!(Meter     <===>        39.3701 Inch      );
impl_from_cf!(Meter     <===> 3.280841666667 Foot      );
impl_from_cf!(Meter     <===> 1.093613888889 Yard      );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Kilometer(pub f64);
pub type Kilometers = Kilometer;

impl_basic_ops!(Kilometer);
impl_div_same!(Kilometer);
impl_scalar_ops!(Kilometer);
impl_unit_debug!(Kilometer => "{}km");
impl_partial_ord!(Kilometer);

impl_through!(Centimeter => Meter => Kilometer);
impl_through!(Millimeter => Meter => Kilometer);
impl_through!(      Inch => Meter => Kilometer);
impl_through!(      Foot => Meter => Kilometer);
impl_through!(      Yard => Meter => Kilometer);
impl_through!(      Mile => Meter => Kilometer);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Centimeter(pub f64);
pub type Centimeters = Centimeter;

impl_basic_ops!(Centimeter);
impl_div_same!(Centimeter);
impl_scalar_ops!(Centimeter);
impl_unit_debug!(Centimeter => "{}cm");
impl_partial_ord!(Centimeter);

impl_through!(Millimeter => Meter => Centimeter);
impl_through!(      Inch => Meter => Centimeter);
impl_through!(      Foot => Meter => Centimeter);
impl_through!(      Yard => Meter => Centimeter);
impl_through!(      Mile => Meter => Centimeter);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Millimeter(pub f64);
pub type Millimeters = Millimeter;

impl_basic_ops!(Millimeter);
impl_div_same!(Millimeter);
impl_scalar_ops!(Millimeter);
impl_unit_debug!(Millimeter => "{}mm");
impl_partial_ord!(Millimeter);

impl_through!(      Inch => Meter => Millimeter);
impl_through!(      Foot => Meter => Millimeter);
impl_through!(      Yard => Meter => Millimeter);
impl_through!(      Mile => Meter => Millimeter);