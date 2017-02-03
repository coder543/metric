use std;

use celsius::Celsius;
use kelvin::Kelvin;

#[derive(Clone, Copy)]
pub struct Fahrenheit(pub f64);

impl_basic_ops!(Fahrenheit);
impl_unit_debug!(Fahrenheit => "{}°F");

impl_from!(Celsius => Fahrenheit, |c| c * 9. / 5. + 32.);
impl_from!(Kelvin  => Fahrenheit, |k| k * 9. / 5. - 459.67);
