//! Defines the imperial length standards as newtypes

use std::{self, fmt};

use metric::*;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Inch(pub f64);
pub type Inches = Inch;

impl_basic_ops!(Inch);
impl_scalar_ops!(Inch);
impl_unit_debug!(Inch => "{}in");
impl_partial_ord!(Inch);
impl_from!(Kilometer  => Inch,   |km|   km / 0.0000254);
impl_from!(Meter      => Inch,    |m|    m / 0.0254);
impl_from!(Centimeter => Inch,   |cm|   cm / 2.54);
impl_from!(Millimeter => Inch,   |mm|   mm / 25.4);
impl_from!(Foot       => Inch, |foot| foot * 12.);
impl_from!(Yard       => Inch, |yard| yard * 36.);
impl_from!(Mile       => Inch, |mile| mile * 63360.);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Foot(pub f64);
pub type Feet = Foot;

impl_basic_ops!(Foot);
impl_scalar_ops!(Foot);
impl_unit_debug!(Foot => "{}ft");
impl_partial_ord!(Foot);
impl_from!(Kilometer  => Foot,   |km|   km / 0.0003048);
impl_from!(Meter      => Foot,    |m|    m / 0.348);
impl_from!(Centimeter => Foot,   |cm|   cm / 30.48);
impl_from!(Millimeter => Foot,   |mm|   mm / 304.8);
impl_from!(Inch       => Foot, |inch| inch / 12.);
impl_from!(Yard       => Foot, |yard| yard *  3.);
impl_from!(Mile       => Foot, |mile| mile * 5280.);


/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Yard(pub f64);
pub type Yards = Yard;

impl_basic_ops!(Yard);
impl_scalar_ops!(Yard);
impl_unit_debug!(Yard => "{}yd");
impl_partial_ord!(Yard);
impl_from!(Kilometer  => Yard,   |km|   km / 0.0009144);
impl_from!(Meter      => Yard,    |m|    m / 0.9144);
impl_from!(Centimeter => Yard,   |cm|   cm / 91.44);
impl_from!(Millimeter => Yard,   |mm|   mm / 914.4);
impl_from!(Inch       => Yard, |inch| inch / 36.);
impl_from!(Foot       => Yard, |foot| foot /  3.);
impl_from!(Mile       => Yard, |mile| mile * 1760.);


/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Mile(pub f64);
pub type Miles = Mile;

impl_basic_ops!(Mile);
impl_scalar_ops!(Mile);
impl_unit_debug!(Mile => "{}mi");
impl_partial_ord!(Mile);
impl_from!(Kilometer  => Mile,   |km|   km / 1.60934);
impl_from!(Meter      => Mile,    |m|    m / 1609.34);
impl_from!(Centimeter => Mile,   |cm|   cm / 160934.);
impl_from!(Millimeter => Mile,   |mm|   mm / 1609340.);
impl_from!(Inch       => Mile, |inch| inch / 63360.);
impl_from!(Foot       => Mile, |foot| foot /  5280.);
impl_from!(Yard       => Mile, |yard| yard /  1760.);
