macro_rules! impl_partial_ord {
    ($impl_type:tt) => {
        impl<T> PartialEq<T> for $impl_type
            where T: Copy + Into<$impl_type>
        {
            fn eq(&self, other: &T) -> bool {
                let other_t: T = *other;
                let other: Self = other_t.into();
                self.0.eq(&other.0)
            }
        }
        impl<T> PartialOrd<T> for $impl_type
            where T: Copy + Into<$impl_type>
        {
            fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
                let other_t: T = *other;
                let other: Self = other_t.into();
                self.0.partial_cmp(&other.0)
            }
        }
    }
}

macro_rules! impl_add {
    ($impl_type:tt) => {
        impl<T> std::ops::Add<T> for $impl_type
            where T: Into<$impl_type>
        {
            type Output = $impl_type;

            fn add(self, other: T) -> Self::Output {
                let other: Self = other.into();
                $impl_type(self.0 + other.0)
            }
        }
    }
}

macro_rules! impl_addassign {
    ($impl_type:tt) => {
        impl<T> std::ops::AddAssign<T> for $impl_type
            where T: Into<$impl_type>
        {
            fn add_assign(&mut self, other: T) {
                let other: Self = other.into();
                self.0 += other.0;
            }
        }
    }
}

macro_rules! impl_sub {
    ($impl_type:tt) => {
        impl<T> std::ops::Sub<T> for $impl_type
            where T: Into<$impl_type>
        {
            type Output = $impl_type;

            fn sub(self, other: T) -> Self::Output {
                let other: Self = other.into();
                $impl_type(self.0 - other.0)
            }
        }
    }
}

macro_rules! impl_subassign {
    ($impl_type:tt) => {
        impl<T> std::ops::SubAssign<T> for $impl_type
            where T: Into<$impl_type>
        {
            fn sub_assign(&mut self, other: T) {
                let other: Self = other.into();
                self.0 -= other.0;
            }
        }
    }
}

macro_rules! impl_mul {
    ($impl_type:tt) => {
        impl<T> std::ops::Mul<T> for $impl_type
            where T: Into<$impl_type>
        {
            type Output = $impl_type;

            fn mul(self, other: T) -> Self::Output {
                let other: Self = other.into();
                $impl_type(self.0 * other.0)
            }
        }
    }
}

macro_rules! impl_mulassign {
    ($impl_type:tt) => {
        impl<T> std::ops::MulAssign<T> for $impl_type
            where T: Into<$impl_type>
        {
            fn mul_assign(&mut self, other: T) {
                let other: Self = other.into();
                self.0 *= other.0;
            }
        }
    }
}

macro_rules! impl_div_same {
    ($impl_type:tt) => {
        impl<T> std::ops::Div<T> for $impl_type
            where T: Into<$impl_type>
        {
            type Output = f64;

            fn div(self, other: T) -> Self::Output {
                let other: Self = other.into();
                self.0 / other.0
            }
        }
    }
}


macro_rules! impl_mul_scalar {
    ($impl_type:tt) => {
        impl std::ops::Mul<f64> for $impl_type
        {
            type Output = $impl_type;

            fn mul(self, other: f64) -> Self::Output {
                $impl_type(self.0 * other)
            }
        }
        impl std::ops::Mul<$impl_type> for f64
        {
            type Output = $impl_type;

            fn mul(self, other: $impl_type) -> Self::Output {
                $impl_type(self * other.0)
            }
        }
        impl std::ops::Mul<i64> for $impl_type
        {
            type Output = $impl_type;

            fn mul(self, other: i64) -> Self::Output {
                $impl_type(self.0 * other as f64)
            }
        }
        impl std::ops::Mul<$impl_type> for i64
        {
            type Output = $impl_type;

            fn mul(self, other: $impl_type) -> Self::Output {
                $impl_type(self as f64 * other.0)
            }
        }
    }
}

macro_rules! impl_mulassign_scalar {
    ($impl_type:tt) => {
        impl std::ops::MulAssign<f64> for $impl_type
        {
            fn mul_assign(&mut self, other: f64) {
                self.0 *= other;
            }
        }
        impl std::ops::MulAssign<i64> for $impl_type
        {
            fn mul_assign(&mut self, other: i64) {
                self.0 *= other as f64;
            }
        }
    }
}

macro_rules! impl_div_scalar {
    ($impl_type:tt) => {
        impl std::ops::Div<f64> for $impl_type
        {
            type Output = $impl_type;

            fn div(self, other: f64) -> Self::Output {
                $impl_type(self.0 / other)
            }
        }
        impl std::ops::Div<i64> for $impl_type
        {
            type Output = $impl_type;

            fn div(self, other: i64) -> Self::Output {
                $impl_type(self.0 / other as f64)
            }
        }
    }
}

macro_rules! impl_divassign_scalar {
    ($impl_type:tt) => {
        impl std::ops::DivAssign<f64> for $impl_type
        {
            fn div_assign(&mut self, other: f64) {
                self.0 /= other;
            }
        }
        impl std::ops::DivAssign<i64> for $impl_type
        {
            fn div_assign(&mut self, other: i64) {
                self.0 /= other as f64;
            }
        }
    }
}

macro_rules! impl_composite_base {
    ($type_a:tt) => {
        impl New for $type_a {
            fn new(val: f64) -> Self {
                $type_a(val)
            }
        }
        impl Tuple for $type_a {
            fn inner(self) -> f64 {
                self.0
            }
        }
        impl<T, U> std::ops::Mul<Mul<T, U>> for $type_a
            where T: Copy + New + Tuple,
        {
            type Output = Mul<$type_a, Mul<T, U>>;

            fn mul(self, other: Mul<T, U>) -> Self::Output {
                Mul($type_a(other.0.inner() * self.0), Mul(T::new(1.0), other.1))
            }
        }
        impl<T, U> std::ops::Mul<Div<T, U>> for $type_a
            where T: Copy + New + Tuple,
        {
            type Output = Mul<$type_a, Div<T, U>>;

            fn mul(self, other: Div<T, U>) -> Self::Output {
                Mul($type_a(other.0.inner() * self.0), Div(T::new(1.0), other.1))
            }
        }
        impl<T, U> std::ops::Div<Mul<T, U>> for $type_a
        {
            type Output = Div<$type_a, Mul<T, U>>;

            fn div(self, other: Mul<T, U>) -> Self::Output {
                Div(self, other)
            }
        }

        impl<T, U> std::ops::Div<Div<T, U>> for $type_a
        {
            type Output = Div<$type_a, Div<T, U>>;

            fn div(self, other: Div<T, U>) -> Self::Output {
                Div(self, other)
            }
        }
        impl<T, U> std::ops::Div<$type_a> for Div<T, U>
            where T: New + Tuple
        {
            type Output = Div<T, Mul<U, $type_a>>;

            fn div(self, other: $type_a) -> Self::Output {
                Div(T::new(self.0.inner() / other.0), Mul(self.1, $type_a(1.0)))
            }
        }
    }
}

macro_rules! impl_composite {
    ($type_a:tt, $type_b:tt) => {
        impl std::ops::Mul<$type_b> for $type_a
        {
            type Output = Mul<$type_a, $type_b>;

            fn mul(self, other: $type_b) -> Self::Output {
                Mul($type_a(self.0 * other.0), $type_b(1.0))
            }
        }
        impl std::ops::Mul<$type_a> for $type_b
        {
            type Output = Mul<$type_b, $type_a>;

            fn mul(self, other: $type_a) -> Self::Output {
                Mul($type_b(self.0 * other.0), $type_a(1.0))
            }
        }
        impl std::ops::Div<$type_b> for $type_a
        {
            type Output = Div<$type_a, $type_b>;

            fn div(self, other: $type_b) -> Self::Output {
                Div($type_a(self.0 / other.0), $type_b(1.0))
            }
        }
        impl std::ops::Div<$type_a> for $type_b
        {
            type Output = Div<$type_b, $type_a>;

            fn div(self, other: $type_a) -> Self::Output {
                Div($type_b(self.0 / other.0), $type_a(1.0))
            }
        }
    }
}

macro_rules! impl_basic_ops {
    ($impl_type:tt) => {
        impl_add!($impl_type);
        impl_addassign!($impl_type);
        impl_sub!($impl_type);
        impl_subassign!($impl_type);
    }
}

macro_rules! impl_scalar_ops {
    ($impl_type:tt) => {
        impl_mul_scalar!($impl_type);
        impl_mulassign_scalar!($impl_type);
        impl_div_scalar!($impl_type);
        impl_divassign_scalar!($impl_type);
    }
}

macro_rules! impl_from {
    ($from_type:tt => $impl_type:tt, $conversion:expr) => {
        impl From<$from_type> for $impl_type {
            fn from(f: $from_type) -> Self {
                $impl_type($conversion(f.0))
            }
        }
        impl<'a> From<&'a $from_type> for $impl_type {
            fn from(f: &'a $from_type) -> Self {
                $impl_type($conversion(f.0))
            }
        }
    }
}

//impl_from for constant factor conversions
macro_rules! impl_from_cf {
    ($from_type:tt <===> $conversion:tt $impl_type:tt) => {
        impl From<$from_type> for $impl_type {
            fn from(f: $from_type) -> Self {
                $impl_type(f.0 * $conversion)
            }
        }
        impl From<$impl_type> for $from_type {
            fn from(f: $impl_type) -> Self {
                $from_type(f.0 / $conversion)
            }
        }
    }
}

//impl_through for conversion by intermediary
macro_rules! impl_through {
    ($from_type:tt => $through_type:tt => $impl_type:tt) => {
        impl From<$from_type> for $impl_type {
            fn from(f: $from_type) -> Self {
                let intermediate: $through_type = f.into();
                intermediate.into()
            }
        }
        impl From<$impl_type> for $from_type {
            fn from(f: $impl_type) -> Self {
                let intermediate: $through_type = f.into();
                intermediate.into()
            }
        }
    }
}

macro_rules! impl_unit_debug {
    ($impl_type:tt => $unitstr:expr) => {
        impl fmt::Debug for $impl_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $unitstr, self.0)
            }
        }
    };
    ($impl_type:tt => $unitstr:expr, $unitstr_plural:expr) => {
        impl fmt::Debug for $impl_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.0 == 1.0 {
                    write!(f, $unitstr, self.0)
                } else {
                    write!(f, $unitstr_plural, self.0)
                }
            }
        }
    }
}
