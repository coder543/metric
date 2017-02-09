extern crate metric;
use std::fmt::Debug;
use metric::temperature::*;

/// will determine whether a given temperature is in danger of
/// reaching the freezing point of water
pub fn in_danger_of_freezing<T>(temp: &T) -> bool
    where T: PartialOrd<Celsius>
{
    *temp < Celsius(5.0)
}

pub fn print_danger<T>(temp: T)
    where T: PartialOrd<Celsius> + Debug
{
    let could_freeze = in_danger_of_freezing(&temp);
    let danger = if !could_freeze { " not" } else { "" };
    println!("{:?} is{} in danger of freezing!", temp, danger);
}

fn main() {
    print_danger(Fahrenheit(34.));
    print_danger(Celsius(3.));
    print_danger(Kelvin(293.));
}
