//! Defines the metric length standards as newtypes

use core::{self, fmt};

use length::imperial::*;
use length::astronomical::*;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Meter(pub f64);
pub type Meters = Meter;
pub type Metre = Meter;
pub type Metres = Meter;

impl_basic_ops!(Meter);
impl_div_same!(Meter);
impl_scalar_ops!(Meter);
impl_unit_debug!(Meter => "{}m");
impl_partial_ord!(Meter);

impl_from_cf!(AU        <===>          149597870700.0     Meter     );
impl_from_cf!(Kilometer <===>                 1_000.0     Meter     );
impl_from_cf!(Mile      <===>                 1_609.344   Meter     );
impl_from_cf!(Meter     <===>                   100.0     Centimeter);
impl_from_cf!(Meter     <===>                 1_000.0     Millimeter);
impl_from_cf!(Meter     <===>             1_000_000.0     Micrometer);
impl_from_cf!(Meter     <===>         1_000_000_000.0     Nanometer );
impl_from_cf!(Meter     <===>     1_000_000_000_000.0     Picometer );
impl_from_cf!(Meter     <===> 1_000_000_000_000_000.0     Femtometer);
impl_from_cf!(Meter     <===>                    39.37007 Inch      );
impl_from_cf!(Meter     <===>          3.280841666667     Foot      );
impl_from_cf!(Meter     <===>          1.093613888889     Yard      );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Kilometer(pub f64);
pub type Kilometers = Kilometer;
pub type Kilometre = Kilometer;
pub type Kilometres = Kilometer;

impl_basic_ops!(Kilometer);
impl_div_same!(Kilometer);
impl_scalar_ops!(Kilometer);
impl_unit_debug!(Kilometer => "{}km");
impl_partial_ord!(Kilometer);

impl_from_cf!(AU        <===>                 149597870.70 Kilometer );
impl_from_cf!(Mile      <===>                     1.609344 Kilometer );
impl_from_cf!(Kilometer <===>                   100_000.00 Centimeter);
impl_from_cf!(Kilometer <===>                 1_000_000.00 Millimeter);
impl_from_cf!(Kilometer <===>             1_000_000_000.00 Micrometer);
impl_from_cf!(Kilometer <===>         1_000_000_000_000.00 Nanometer );
impl_from_cf!(Kilometer <===>     1_000_000_000_000_000.00 Picometer );
impl_from_cf!(Kilometer <===> 1_000_000_000_000_000_000.00 Femtometer);
impl_from_cf!(Kilometer <===>                    39_370.07 Inch      );
impl_from_cf!(Kilometer <===>                     3_280.84 Foot      );
impl_from_cf!(Kilometer <===>                     1_093.61 Yard      );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Centimeter(pub f64);
pub type Centimeters = Centimeter;
pub type Centimetre = Centimeter;
pub type Centimetres = Centimeter;

impl_basic_ops!(Centimeter);
impl_div_same!(Centimeter);
impl_scalar_ops!(Centimeter);
impl_unit_debug!(Centimeter => "{}cm");
impl_partial_ord!(Centimeter);

impl_from_cf!(Centimeter <===>                 10.00 Millimeter);
impl_from_cf!(Centimeter <===>             10_000.00 Micrometer);
impl_from_cf!(Centimeter <===>         10_000_000.00 Nanometer );
impl_from_cf!(Centimeter <===>     10_000_000_000.00 Picometer );
impl_from_cf!(Centimeter <===> 10_000_000_000_000.00 Femtometer);
impl_from_cf!(Inch       <===>                  2.54 Centimeter);
impl_from_cf!(Foot       <===>                 30.48 Centimeter);
impl_from_cf!(Yard       <===>                 91.44 Centimeter);
impl_from_cf!(Mile       <===>             160934.40 Centimeter);
impl_from_cf!(AU         <===>     14959787070000.00 Centimeter);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Millimeter(pub f64);
pub type Millimeters = Millimeter;
pub type Millimetre = Millimeter;
pub type Millimetres = Millimeter;

impl_basic_ops!(Millimeter);
impl_div_same!(Millimeter);
impl_scalar_ops!(Millimeter);
impl_unit_debug!(Millimeter => "{}mm");
impl_partial_ord!(Millimeter);

impl_from_cf!(Millimeter <===>             1_000.0 Micrometer);
impl_from_cf!(Millimeter <===>         1_000_000.0 Nanometer );
impl_from_cf!(Millimeter <===>     1_000_000_000.0 Picometer );
impl_from_cf!(Millimeter <===> 1_000_000_000_000.0 Femtometer);
impl_from_cf!(Inch       <===>                25.4 Millimeter);
impl_from_cf!(Foot       <===>               304.8 Millimeter);
impl_from_cf!(Yard       <===>               914.4 Millimeter);
impl_from_cf!(Mile       <===>           1609344.0 Millimeter);
impl_from_cf!(AU         <===>   149597870700000.0 Millimeter);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Micrometer(pub f64);
pub type Micrometers = Micrometer;
pub type Micrometre = Micrometer;
pub type Micrometres = Micrometer;

impl_basic_ops!(Micrometer);
impl_div_same!(Micrometer);
impl_scalar_ops!(Micrometer);
impl_unit_debug!(Micrometer => "{}um");
impl_partial_ord!(Micrometer);

impl_from_cf!(Micrometer <===>              1_000.0 Nanometer );
impl_from_cf!(Micrometer <===>          1_000_000.0 Picometer );
impl_from_cf!(Micrometer <===>      1_000_000_000.0 Femtometer);
impl_from_cf!(Inch       <===>              25400.0 Micrometer);
impl_from_cf!(Foot       <===>             304800.0 Micrometer);
impl_from_cf!(Yard       <===>             914400.0 Micrometer);
impl_from_cf!(Mile       <===>         1609344000.0 Micrometer);
impl_from_cf!(AU         <===> 149597870700000000.0 Micrometer);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Nanometer(pub f64);
pub type Nanometers = Nanometer;
pub type Nanometre = Nanometer;
pub type Nanometres = Nanometer;


impl_basic_ops!(Nanometer);
impl_div_same!(Nanometer);
impl_scalar_ops!(Nanometer);
impl_unit_debug!(Nanometer => "{}um");
impl_partial_ord!(Nanometer);

impl_from_cf!(Nanometer  <===>                  1000.0 Picometer );
impl_from_cf!(Nanometer  <===>               1000000.0 Femtometer);
impl_from_cf!(Inch       <===>              25400000.0 Nanometer );
impl_from_cf!(Foot       <===>             304800000.0 Nanometer );
impl_from_cf!(Yard       <===>             914400000.0 Nanometer );
impl_from_cf!(Mile       <===>         1609344000000.0 Nanometer );
impl_from_cf!(AU         <===> 149597870700000000000.0 Nanometer );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Picometer(pub f64);
pub type Picometers = Picometer;
pub type Picometre = Picometer;
pub type Picometres = Picometer;

impl_basic_ops!(Picometer);
impl_div_same!(Picometer);
impl_scalar_ops!(Picometer);
impl_unit_debug!(Picometer => "{}pm");
impl_partial_ord!(Picometer);

impl_from_cf!(Picometer  <===>                     1000.0 Femtometer);
impl_from_cf!(Inch       <===>              25400000000.0 Picometer );
impl_from_cf!(Foot       <===>             304800000000.0 Picometer );
impl_from_cf!(Yard       <===>             914400000000.0 Picometer );
impl_from_cf!(Mile       <===>         1609344000000000.0 Picometer );
impl_from_cf!(AU         <===> 149597870700000000000000.0 Picometer );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Femtometer(pub f64);
pub type Femtometers = Femtometer;
pub type Femtometre = Femtometer;
pub type Femtometres = Femtometer;

impl_basic_ops!(Femtometer);
impl_div_same!(Femtometer);
impl_scalar_ops!(Femtometer);
impl_unit_debug!(Femtometer => "{}fm");
impl_partial_ord!(Femtometer);

impl_from_cf!(Inch       <===>              25400000000000.0 Femtometer);
impl_from_cf!(Foot       <===>             304800000000000.0 Femtometer);
impl_from_cf!(Yard       <===>             914400000000000.0 Femtometer);
impl_from_cf!(Mile       <===>         1609344000000000000.0 Femtometer);
impl_from_cf!(AU         <===> 149597870700000000000000000.0 Femtometer);