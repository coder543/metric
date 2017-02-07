//! This crate is currently a binary crate, but it will soon switch to a library crate.

#![warn(unused_results, missing_docs, unused_extern_crates)]
#![deny(future_incompatible,)]

#[macro_use]
mod implmacro;

pub mod temperature;
use temperature::*;

pub mod length;
use length::*;

/// will determine whether a given temperature is in danger of
/// reaching the freezing point of water
pub fn danger_of_freezing<T>(temp: T) -> bool
    where T: PartialOrd<Celsius>
{
    temp < Celsius(5.0)
}

fn main() {
    let outside_temperature = Fahrenheit(34.);
    let danger = if !danger_of_freezing(outside_temperature) {
        " not"
    } else {
        ""
    };
    println!("{:?} is{} in danger of freezing!",
             outside_temperature,
             danger);

    let total_length = Meter(3.0) + Inch(42.0);

    println!("{:?}", total_length);
}
