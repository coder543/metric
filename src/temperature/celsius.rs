use std;

use fahrenheit::Fahrenheit;
use kelvin::Kelvin;

#[derive(Clone, Copy)]
pub struct Celsius(pub f64);

impl_basic_ops!(Celsius);
impl_unit_debug!(Celsius => "{}°C");

impl_from!(Fahrenheit => Celsius, |f| (f -  32.) * 5. / 9.);
impl_from!(Kelvin     => Celsius, |k|  k - 273.);
