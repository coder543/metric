
use composite::*;
use mass::metric::Kilogram;
use length::metric::Meter;
use time::Second;

pub type Newton = Mul<Kilogram, Div<Meter, Mul<Second, Second>>>;
