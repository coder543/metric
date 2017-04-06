extern crate metric;
use metric::length::metric::*;
use metric::temperature::Fahrenheit;
use metric::mass::imperial::Ounce;

fn main() {
    let stadium = Meter(225.);
    let temp = Fahrenheit(45.);
    let meters_per_degree = stadium/temp;
    let ounce_meters_per_degree = Ounce(23.) * meters_per_degree;

    println!("{:?}", ounce_meters_per_degree);
}