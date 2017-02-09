extern crate metric;
use metric::length::*;

fn main() {
    let endzone = Yard(10.);
    let football_field = Yard(100.) + endzone * 2.;
    let stadium = Meter(225.);
    let edge_of_field_to_stadium_exterior = (stadium - football_field) / 2.;
    println!("({:?} - {:?}) / 2 = {:?}",
             stadium,
             football_field,
             edge_of_field_to_stadium_exterior);
}