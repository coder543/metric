
#[macro_use]
mod implmacro;

pub mod temperature;
use temperature::*;

/// will determine whether a given temperature is in danger of
/// reaching the freezing point of water
pub fn danger_of_freezing<T>(temp: T) -> bool
    where T: PartialOrd<Celsius>
{
    return temp < Celsius(5.0);
}

pub fn main() {
    let outside_temperature = Fahrenheit(34.);
    let danger = if !danger_of_freezing(outside_temperature) {
        " not"
    } else {
        ""
    };
    println!("{:?} is{} in danger of freezing!",
             outside_temperature,
             danger);
}
