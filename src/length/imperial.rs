//! Defines the imperial length standards as newtypes

use core::{self, fmt};

use composite::UnitName;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Inch(pub f64);
pub type Inches = Inch;

impl_basic_ops!(Inch);
impl_div_same!(Inch);
impl_scalar_ops!(Inch);
impl_unit_debug!(Inch => "in");
impl_partial_ord!(Inch);

impl_from_cf!(Foot <===>    12. Inch);
impl_from_cf!(Yard <===>    36. Inch);
impl_from_cf!(Mile <===> 63360. Inch);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Foot(pub f64);
pub type Feet = Foot;

impl_basic_ops!(Foot);
impl_div_same!(Foot);
impl_scalar_ops!(Foot);
impl_unit_debug!(Foot => "ft");
impl_partial_ord!(Foot);

impl_from_cf!(Yard <===>    3. Foot);
impl_from_cf!(Mile <===> 5280. Foot);


/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Yard(pub f64);
pub type Yards = Yard;

impl_basic_ops!(Yard);
impl_div_same!(Yard);
impl_scalar_ops!(Yard);
impl_unit_debug!(Yard => "yd");
impl_partial_ord!(Yard);

impl_from_cf!(Mile <===> 1760. Yard);


/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Mile(pub f64);
pub type Miles = Mile;

impl_basic_ops!(Mile);
impl_div_same!(Mile);
impl_scalar_ops!(Mile);
impl_unit_debug!(Mile => "mi");
impl_partial_ord!(Mile);
