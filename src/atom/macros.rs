#![macro_use]

#[macro_export]
macro_rules! int_explicit_conversion {
    ($ta:ty, $tb:ty, $ti:expr) => (
        impl From<$ta> for $tb {
            fn from(num: $ta) -> $tb {
                $ti(num as i64)
            }
        }
    )
}

#[macro_export]
macro_rules! float_explicit_conversion {
    ($ta:ty, $tb:tt, $ti:ty) => (
        impl From<$ta> for $tb {
            fn from(num: $ta) -> $tb {
                $tb::from(num.to_string().as_str())
            }
        }
    )
}

#[macro_export]
macro_rules! numeric_int_explicit_conversion {
    ($ta:ty, $tb:expr, $ti:ty) => (
        impl From<$ta> for $ti {
            fn from(num: $ta) -> SimplexAtom {
                $tb(Numeric::LittleInteger(num as i64))
            }
        }
    )
}

#[macro_export]
macro_rules! numeric_float_explicit_conversion {
    ($ta:ty, $tb:expr, $ti:ty) => (
        impl From<$ta> for $ti {
            fn from(num: $ta) -> SimplexAtom {
                $tb(Numeric::from(num.to_string().as_str()))
            }
        }
    )
}
