use std::io;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::borrow::Cow;

use parsing::utilities::numerics::get_representable_integer;
use atom::numbers::number::Numeric;

#[macro_use]
use atom::macros;

extern crate decimal;
use decimal::d128;

extern crate num;
use num::{ToPrimitive, FromPrimitive};
use std::str::FromStr;

impl<'a> From<&'a str> for Numeric {
    fn from(s: &str) -> Numeric {
        match s.parse::<i64>() {
            Ok(num) => Numeric::LittleInteger(num),
            Err(_) => {
                match d128::from_str(s) {
                    Ok(num) => {
                        if num.to_string() != "NaN" {
                            Numeric::LittleReal(num)
                        } else {
                            Numeric::NaN
                        }
                    }
                    Err(_) => Numeric::NaN,
                }
            }
        }
    }
}

int_explicit_conversion!(i8, Numeric, Numeric::LittleInteger);
int_explicit_conversion!(i16, Numeric, Numeric::LittleInteger);
int_explicit_conversion!(i32, Numeric, Numeric::LittleInteger);
int_explicit_conversion!(i64, Numeric, Numeric::LittleInteger);
int_explicit_conversion!(u8, Numeric, Numeric::LittleInteger);
int_explicit_conversion!(u16, Numeric, Numeric::LittleInteger);
int_explicit_conversion!(u32, Numeric, Numeric::LittleInteger);

float_explicit_conversion!(f32, Numeric, Numeric::LittleInteger);
float_explicit_conversion!(f64, Numeric, Numeric::LittleInteger);
