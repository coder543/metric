use std;

use fahrenheit::Fahrenheit;
use celsius::Celsius;

#[derive(Clone, Copy)]
pub struct Kelvin(pub f64);

impl_basic_ops!(Kelvin);
impl_unit_debug!(Kelvin => "{}K");

impl_from!(Celsius    => Kelvin, |c|  c + 273.);
impl_from!(Fahrenheit => Kelvin, |f| (f + 459.67) * 5. / 9.);
