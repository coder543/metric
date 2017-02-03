
#[macro_use]
mod implmacro;

mod temperature;
use temperature::*;

// T is anything which can be turned into Celsius
fn danger_of_freezing<T>(temp: T) -> bool
    where T: Into<Celsius>
{
    let temperature: Celsius = temp.into();
    return temperature.0 < 5.0;
}

pub fn main() {
    let x = Fahrenheit(34.);
    let danger = if !danger_of_freezing(x) { " not" } else { "" };
    println!("{:?} is{} in danger of freezing!", x, danger);
}
