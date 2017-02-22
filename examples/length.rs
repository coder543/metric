extern crate metric;
use metric::length::imperial::*;
use metric::length::astronomical::*;
use metric::length::metric::*;

fn main() {
    let endzone = Yard(10.);
    let football_field = Yard(100.) + 2 * endzone;
    let stadium = Meter(225.);
    let edge_of_field_to_stadium_exterior = (stadium - football_field) / 2.;
    println!("({:?} - {:?}) / 2 = {:?}",
             stadium,
             football_field,
             edge_of_field_to_stadium_exterior);

    let earth_sun: AU = Kilometer(149597870.700).into();
    let earth_sun_m: Meters = earth_sun.into();
    let earth_sun_mi: Miles = earth_sun.into();
    println!("{:?}\n{:?}\n{:?}", earth_sun, earth_sun_m, earth_sun_mi);
}