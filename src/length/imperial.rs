//! Defines the imperial length standards as newtypes

use core::{self, fmt};

use composite::UnitName;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Inch(pub f64);
pub type Inches = Inch;

impl_full_unit!(Inch);
impl_unit_debug!(Inch => "in");

impl_from_cf!(Foot <===>    12. Inch);
impl_from_cf!(Yard <===>    36. Inch);
impl_from_cf!(Mile <===> 63360. Inch);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Foot(pub f64);
pub type Feet = Foot;

impl_full_unit!(Foot);
impl_unit_debug!(Foot => "ft");

impl_from_cf!(Yard <===>    3. Foot);
impl_from_cf!(Mile <===> 5280. Foot);


/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Yard(pub f64);
pub type Yards = Yard;

impl_full_unit!(Yard);
impl_unit_debug!(Yard => "yd");

impl_from_cf!(Mile <===> 1760. Yard);


/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Mile(pub f64);
pub type Miles = Mile;

impl_full_unit!(Mile);
impl_unit_debug!(Mile => "mi");
