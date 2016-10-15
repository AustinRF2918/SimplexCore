use std::io;
use std::ops::{Add, Sub, Mul, Div};

extern crate decimal;
use decimal::d128;

extern crate num;
use num::{ToPrimitive, FromPrimitive};
use std::str::FromStr;

pub enum Numeric {
    LittleInteger(i64),
    LittleReal(d128),
}

impl Numeric {
    pub fn from_str(s: &str) -> Option<Numeric> {
        match s.parse::<i64>() {
            Ok(num) => {
                Some(Numeric::LittleInteger(num))
            }, Err(_) => {
                match d128::from_str(s) {
                    Ok(num) => {
                        Some(Numeric::LittleReal(num))
                    }, Err(_) => {
                        None
                    }
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &Numeric::LittleInteger(i) => {
                format!("Simplex`Integer[{}]", i)
            }, &Numeric::LittleReal(ref r) => {
                format!("Simplex`Real[{}]", r)
            }
        }
    }

    pub fn simplify(self) -> Numeric {
        match self {
            Numeric::LittleReal(r) => {
                if r.is_integer() {
                    let new_integer = (r.to_string().as_str()).parse::<i64>().unwrap();
                    let mut new_self = Numeric::LittleInteger(new_integer);
                    new_self
                } else {
                    self
                }
            }
            _ => {
                self
            }
        }
    }

    pub fn is_nan(&self) -> bool {
        match self {
            &Numeric::LittleInteger(_) => {
                false
            }, &Numeric::LittleReal(ref r) => {
                r.to_string() == "NaN" 
            }
        }
    }

    pub fn head(&self) -> &str {
        match self {
            &Numeric::LittleInteger(_) => {
                "Simplex`Integer"
            }, &Numeric::LittleReal(_) => {
                "Simplex`Real"
            }
        }
    }
}
impl Add for Numeric {
    type Output = Numeric;

    fn add(self, other: Numeric) -> Numeric {
        //TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs + rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() + rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs + d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs + rhs).simplify()
            }
        }
    }
}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs - rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() - rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs - d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs - rhs).simplify()
            }
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, other: Numeric) -> Numeric {
        //TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs * rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() * rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs * d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs * rhs).simplify()
            }
        }
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, other: Numeric) -> Numeric {
        //TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                //Unsafe, very very unsafe.
                // This should use two d128s wrapped from lhs and rhs as division.
                Numeric::LittleReal(d128::from_str((lhs as f64 / rhs as f64).to_string().as_str()).unwrap())
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() / rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs / d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs / rhs).simplify()
            }
        }
    }
}
