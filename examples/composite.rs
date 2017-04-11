extern crate metric;
use metric::composite::*;
use metric::length::metric::*;
use metric::length::imperial::Mile;
use metric::time::*;

pub type MPH = Div<Mile, Hour>;
pub type MPS = Div<Meter, Second>;

fn calc_accel<Time>(speed1: MPH, speed2: MPH, timediff: Time) -> Div<Meter, Mul<Second, Second>>
    where Time: Into<Second>
{
    let diff = speed2 - speed1;

    //types
    //diff.0: Mile
    //diff.1: Hour
    //therefore, using ::from is safe. Rust will only let you
    //use ::from on a value of appropriate type. No chance of misconversion.
    let diff = Meter::from(diff.0) / Second::from(Hour(1.0));

    //another way to convert is using .into()
    let timediff: Second = timediff.into();

    return diff / timediff;
}

fn main() {
    let speed: MPH = Mile(35.) / Hour(0.7);
    let speed2: MPH = Mile(60.) / Hour(0.65);
    let accel = calc_accel(speed, speed2, Second(30.));
    println!("{:?} -> {:?} is an acceleration of {:?}",
             speed,
             speed2,
             accel);
}
