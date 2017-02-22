//! Defines the metric mass standards as newtypes

use std::{self, fmt};

use mass::imperial::{Ounce, Pound, Ton as ITon};

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Ton(pub f64);
pub type Tons = Ton;

impl_basic_ops!(Ton);
impl_div_same!(Ton);
impl_scalar_ops!(Ton);
impl_unit_debug!(Ton => "{}mT");
impl_partial_ord!(Ton);

impl_from_cf!(Ton <===>          1.102311 ITon);
impl_from_cf!(Ton <===>       2204.62262 Pound);
impl_from_cf!(Ton <===>      35273.9619  Ounce);
impl_from_cf!(Ton <===>       1000.0  Kilogram);
impl_from_cf!(Ton <===>    1000000.0      Gram);
impl_from_cf!(Ton <===>  100000000.0 Centigram);
impl_from_cf!(Ton <===> 1000000000.0 Milligram);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Kilogram(pub f64);
pub type Kilograms = Kilogram;

impl_basic_ops!(Kilogram);
impl_div_same!(Kilogram);
impl_scalar_ops!(Kilogram);
impl_unit_debug!(Kilogram => "{}kg");
impl_partial_ord!(Kilogram);

impl_from_cf!(Kilogram <===>     1000.0     Gram );
impl_from_cf!(Kilogram <===>   100000.0 Centigram);
impl_from_cf!(Kilogram <===>  1000000.0 Milligram);
impl_from_cf!(Kilogram <===> 2.20462262     Pound);
impl_from_cf!(Kilogram <===> 35.2739619     Ounce);
impl_from_cf!(ITon     <===>   907.1847  Kilogram);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Gram(pub f64);
pub type Grams = Gram;

impl_basic_ops!(Gram);
impl_div_same!(Gram);
impl_scalar_ops!(Gram);
impl_unit_debug!(Gram => "{}g");
impl_partial_ord!(Gram);

impl_from_cf!(Gram  <===>    100.0  Centigram);
impl_from_cf!(Gram  <===>   1000.0  Milligram);
impl_from_cf!(Pound <===>    453.592370  Gram);
impl_from_cf!(Ounce <===>     28.3495231 Gram);
impl_from_cf!(ITon  <===> 907184.7       Gram);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Centigram(pub f64);
pub type Centigrams = Centigram;

impl_basic_ops!(Centigram);
impl_div_same!(Centigram);
impl_scalar_ops!(Centigram);
impl_unit_debug!(Centigram => "{}cg");
impl_partial_ord!(Centigram);

impl_from_cf!(Centigram <===>    10.00000 Milligram);
impl_from_cf!(Pound     <===> 45359.23700 Centigram);
impl_from_cf!(Ounce     <===>  2834.95231 Centigram);
impl_from_cf!(ITon      <===>  90718470.0 Centigram);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Milligram(pub f64);
pub type Milligrams = Milligram;

impl_basic_ops!(Milligram);
impl_div_same!(Milligram);
impl_scalar_ops!(Milligram);
impl_unit_debug!(Milligram => "{}mg");
impl_partial_ord!(Milligram);

impl_from_cf!(Pound     <===> 453592.3700 Milligram);
impl_from_cf!(Ounce     <===>  28349.5231 Milligram);
impl_from_cf!(ITon      <===> 907184700.0 Milligram);
