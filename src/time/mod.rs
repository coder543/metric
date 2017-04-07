use core::{self, fmt};

use composite::UnitName;

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Second(pub f64);
pub type Seconds = Second;

impl_full_unit!(Second);
impl_unit_debug!(Second => "s");

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Minute(pub f64);
pub type Minutes = Minute;

impl_full_unit!(Minute);
impl_unit_debug!(Minute => "min");

impl_from_cf!(Minute <===> 60.0 Second);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Hour(pub f64);
pub type Hours = Hour;

impl_full_unit!(Hour);
impl_unit_debug!(Hour => "hr");

impl_from_cf!(Hour <===> 3600.0 Second);
impl_from_cf!(Hour <===>   60.0 Minute);


/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Day(pub f64);
pub type Days = Day;

impl_full_unit!(Day);
impl_unit_debug!(Day => "d");

impl_from_cf!(Day <===> 86400.0 Second);
impl_from_cf!(Day <===>  1440.0 Minute);
impl_from_cf!(Day <===>    24.0 Hour  );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Year(pub f64);
pub type Years = Year;

impl_full_unit!(Year);
impl_unit_debug!(Year => "yr", "{}yrs");

impl_from_cf!(Year <===> 31556925.9    Second);
impl_from_cf!(Year <===>   525948.76   Minute);
impl_from_cf!(Year <===>     8765.812  Hour  );
impl_from_cf!(Year <===>      365.2421 Day   );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Decade(pub f64);
pub type Decades = Decade;

impl_full_unit!(Decade);
impl_unit_debug!(Decade => " decade", "{} decades");

impl_from_cf!(Decade <===> 315569259.0   Second);
impl_from_cf!(Decade <===>   5259487.6   Minute);
impl_from_cf!(Decade <===>     87658.12  Hour  );
impl_from_cf!(Decade <===>      3652.421 Day   );
impl_from_cf!(Decade <===>        10.0   Year  );

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Century(pub f64);
pub type Centurys = Century;

impl_full_unit!(Century);
impl_unit_debug!(Century => " century", "{} centuries");

impl_from_cf!(Century <===> 3155692590.0    Second);
impl_from_cf!(Century <===>   52594876.0    Minute);
impl_from_cf!(Century <===>     876581.2    Hour  );
impl_from_cf!(Century <===>      36524.21   Day   );
impl_from_cf!(Century <===>        100.0    Year  );
impl_from_cf!(Century <===>         10.0    Decade);

/// A newtype that wraps around `f64` and provides convenience functions for unit-aware and type-safe manipulation.
#[derive(Clone, Copy)]
pub struct Millennium(pub f64);
pub type Millennia = Millennium;

impl_full_unit!(Millennium);
impl_unit_debug!(Millennium => " millennium", "{} millennia");

impl_from_cf!(Millennium <===> 31556925900.0    Second );
impl_from_cf!(Millennium <===>   525948760.0    Minute );
impl_from_cf!(Millennium <===>     8765812.0    Hour   );
impl_from_cf!(Millennium <===>      365242.1    Day    );
impl_from_cf!(Millennium <===>        1000.0    Year   );
impl_from_cf!(Millennium <===>         100.0    Decade );
impl_from_cf!(Millennium <===>          10.0    Century);