use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use std::borrow::Cow;
use parsing::utilities::numerics::get_representable_integer;

extern crate decimal;
use decimal::d128;

extern crate num;
use std::str::FromStr;

// TODO: Make emulated integer using d128.
#[derive(Copy, Clone, Debug)]
pub enum Numeric {
    LittleInteger(i64),
    LittleReal(d128),
    NaN,
}

impl Numeric {
    pub fn as_str<'a>(&'a self) -> Cow<'a, str> {
        match self {
            &Numeric::LittleInteger(i) => Cow::Owned(i.to_string()),
            &Numeric::LittleReal(ref r) => Cow::Owned(r.to_string()),
            &Numeric::NaN => Cow::Borrowed("NaN"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &Numeric::LittleInteger(i) => format!("{}", i),
            &Numeric::LittleReal(ref r) => format!("{}", r),
            &Numeric::NaN => format!("NaN"),
        }
    }

    pub fn simplify(self) -> Numeric {
        match self {
            Numeric::LittleReal(r) => {
                let real_number = r.to_string();

                match get_representable_integer(real_number.as_str()) {
                    Some(num) => Numeric::LittleInteger(num),

                    None => {
                        if real_number.contains("NaN") {
                            Numeric::NaN
                        } else {
                            self
                        }
                    }
                }
            }
            _ => self,
        }
    }

    #[allow(dead_code)]
    pub fn capacity(&self) -> usize {
        match self {
            &Numeric::LittleInteger(_) => 64,
            &Numeric::LittleReal(_) => 128,
            &Numeric::NaN => 8,
        }
    }
}

impl Eq for Numeric {}

impl PartialEq for Numeric {
    fn eq(&self, other: &Numeric) -> bool {
        match (self, other) {
            (&Numeric::NaN, &Numeric::NaN) => true,
            (&Numeric::NaN, _) => false,
            (_, &Numeric::NaN) => false,
            (&Numeric::LittleInteger(lhs), &Numeric::LittleInteger(rhs)) => lhs == rhs,
            (&Numeric::LittleInteger(_), &Numeric::LittleReal(_)) => *self == other.simplify(),
            (&Numeric::LittleReal(_), &Numeric::LittleInteger(_)) => other.simplify() == *self,
            (&Numeric::LittleReal(lhs), &Numeric::LittleReal(rhs)) => lhs == rhs,

        }
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, other: Numeric) -> Numeric {
        // TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::NaN, _) => Numeric::NaN,
            (_, Numeric::NaN) => Numeric::NaN,
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs + rhs)
            }
            (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() + rhs)
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs + d128::from_str(rhs.to_string().as_str()).unwrap())
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs + rhs).simplify()
            }
        }
    }
}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::NaN, _) => Numeric::NaN,
            (_, Numeric::NaN) => Numeric::NaN,
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs - rhs)
            }
            (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() - rhs)
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs - d128::from_str(rhs.to_string().as_str()).unwrap())
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs - rhs).simplify()
            }
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, other: Numeric) -> Numeric {
        // TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::NaN, _) => Numeric::NaN,
            (_, Numeric::NaN) => Numeric::NaN,
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs * rhs)
            }
            (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() * rhs)
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs * d128::from_str(rhs.to_string().as_str()).unwrap())
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs * rhs).simplify()
            }
        }
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, other: Numeric) -> Numeric {
        // TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::NaN, _) => Numeric::NaN,
            (_, Numeric::NaN) => Numeric::NaN,
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                // Unsafe, very very unsafe.
                // This should use two d128s wrapped from lhs and rhs as division.
                Numeric::LittleReal(d128::from_str((lhs as f64 / rhs as f64).to_string().as_str())
                        .unwrap())
                    .simplify()
            }
            (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() / rhs)
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs / d128::from_str(rhs.to_string().as_str()).unwrap())
                    .simplify()
            }
            (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs / rhs).simplify()
            }
        }
    }
}
