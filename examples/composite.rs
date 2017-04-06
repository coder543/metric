extern crate metric;
use metric::length::metric::*;
use metric::temperature::Fahrenheit;
use metric::mass::imperial::Ounce;

use metric::composite::*;

use metric::length::imperial::Mile;
use metric::time::*;
pub type MPH = Div<Mile, Hour>;

fn main() {
    let stadium = Meter(225.);
    let temp = Fahrenheit(45.);
    let meters_per_degree = stadium/temp;
    let ounce_meters_per_degree: Mul<Ounce, Div<Meter, Fahrenheit>> = Ounce(23.) * meters_per_degree;
    let speed: MPH = Mile(35.) / Hour(0.7);
    let something = speed * meters_per_degree;
    println!("{:?}", meters_per_degree);
    println!("{:?}", ounce_meters_per_degree);
    println!("speed: {:?}", speed);
    println!("something: {:?}", something);
}