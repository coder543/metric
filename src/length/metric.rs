//! Defines the metric length standards as newtypes

use std::{self, fmt};

use imperial::*;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Meter(pub f64);
pub type Meters = Meter;

impl_basic_ops!(Meter);
impl_scalar_ops!(Meter);
impl_unit_debug!(Meter => "{}m");
impl_partial_ord!(Meter);
impl_from!(Kilometer  => Meter,   |km|   km *  1_000.);
impl_from!(Centimeter => Meter,   |cm|   cm /    100.);
impl_from!(Millimeter => Meter,   |mm|   mm /  1_000.);
impl_from!(Inch       => Meter, |inch| inch *  0.0254);
impl_from!(Foot       => Meter, |foot| foot *  0.3048);
impl_from!(Yard       => Meter, |yard| yard *  0.9144);
impl_from!(Mile       => Meter, |mile| mile * 1609.34);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Kilometer(pub f64);
pub type Kilometers = Kilometer;

impl_basic_ops!(Kilometer);
impl_scalar_ops!(Kilometer);
impl_unit_debug!(Kilometer => "{}km");
impl_partial_ord!(Kilometer);

impl_from!(Meter      => Kilometer,    |m|    m / 1_000.);
impl_from!(Centimeter => Kilometer,   |cm|   cm / 100_000.);
impl_from!(Millimeter => Kilometer,   |mm|   mm / 1_000_000.);
impl_from!(Inch       => Kilometer, |inch| inch * 0.0000254);
impl_from!(Foot       => Kilometer, |foot| foot * 0.0003048);
impl_from!(Yard       => Kilometer, |yard| yard * 0.0009144);
impl_from!(Mile       => Kilometer, |mile| mile * 1.60934);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Centimeter(pub f64);
pub type Centimeters = Centimeter;

impl_basic_ops!(Centimeter);
impl_scalar_ops!(Centimeter);
impl_unit_debug!(Centimeter => "{}cm");
impl_partial_ord!(Centimeter);

impl_from!(Kilometer  => Centimeter, |km| km * 100_000.);
impl_from!(Meter      => Centimeter,  |m|  m * 100.);
impl_from!(Millimeter => Centimeter, |mm| mm / 10.);
impl_from!(Inch       => Centimeter, |inch| inch *  2.54);
impl_from!(Foot       => Centimeter, |foot| foot *  30.48);
impl_from!(Yard       => Centimeter, |yard| yard *  91.44);
impl_from!(Mile       => Centimeter, |mile| mile * 160934.);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Millimeter(pub f64);
pub type Millimeters = Millimeter;

impl_basic_ops!(Millimeter);
impl_scalar_ops!(Millimeter);
impl_unit_debug!(Millimeter => "{}mm");
impl_partial_ord!(Millimeter);

impl_from!(Kilometer  => Millimeter, |km| km * 1_000_000.);
impl_from!(Meter      => Millimeter,  |m|  m * 1000.);
impl_from!(Centimeter => Millimeter, |cm| cm * 10.);
impl_from!(Inch       => Millimeter, |inch| inch *  25.4);
impl_from!(Foot       => Millimeter, |foot| foot *  304.8);
impl_from!(Yard       => Millimeter, |yard| yard *  914.4);
impl_from!(Mile       => Millimeter, |mile| mile * 1609340.);