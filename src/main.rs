//! This crate is currently a binary crate, but it will soon switch to a library crate.

#![warn(unused_results, unused_extern_crates)]
#![deny(future_incompatible,)]

#[macro_use]
mod implmacro;

pub mod temperature;
use temperature::*;

pub mod length;
use length::*;

pub mod time;
pub mod mass;
pub mod electrical;

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

    let endzone = Yard(10.);
    let football_field = Yard(100.) + endzone * 2.;
    let stadium = Meter(225.);
    let edge_of_field_to_stadium_exterior = (stadium - football_field) / 2.;
    println!("({:?} - {:?}) / 2 = {:?}",
             stadium,
             football_field,
             edge_of_field_to_stadium_exterior);
}
