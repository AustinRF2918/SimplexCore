use std::io;
use std::ops::{Add, Sub};
use atom::numbers::fixed_point::I32p32;
use decimal::*;

extern crate num;
use num::{ToPrimitive, FromPrimitive};

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
                match s.parse::<f64>() {
                    Ok(num) => {
                        Some(Numeric::LittleReal(d128!(num)))
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
                // Unsafe.
                format!("Simplex`Real[{}]", r)
            }
        }
    }

    pub fn head(&self) -> String {
        match self {
            &Numeric::LittleInteger(_) => {
                "Simplex`Integer".to_string()
            }, &Numeric::LittleReal(_) => {
                "Simplex`Real".to_string()
            }
        }
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs + rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                let lhs_real = I48p16::from_i64(lhs).unwrap();
                Numeric::LittleReal(lhs_real + rhs)
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                let rhs_real = I48p16::from_i64(rhs).unwrap();
                Numeric::LittleReal(rhs_real + lhs)
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs + rhs)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod parsing_tests {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_parses_int() {
            let x = Numeric::from_str("55").unwrap();
            assert_eq!(x.head(), "Simplex`Integer".to_string());
        }

        #[test]
        fn it_doesnt_parse_bad_int() {
            let x = Numeric::from_str("dsajfdksjk");
            assert_eq!(x.is_some(), false);
        }

        #[test]
        fn it_parses_real() {
            let x = Numeric::from_str("55.552").unwrap();
            assert_eq!(x.head(), "Simplex`Real".to_string());
        }

        #[test]
        fn it_doesnt_parse_bad_real() {
            let x = Numeric::from_str("dsajfd.ksjk");
            assert_eq!(x.is_some(), false);
        }
    }

    mod formatting_tests {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_shows_int() {
            let x = Numeric::from_str("55").unwrap();
            assert_eq!(x.to_string(), "Simplex`Integer[55]".to_string());
        }

        #[test]
        fn it_shows_real() {
            let x = Numeric::from_str("55.55").unwrap();
            assert_eq!(x.to_string(), "Simplex`Real[55.55]".to_string());
        }
    }

    mod low_sized_addition {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_add_int_int() {
            let x = Numeric::from_str("55").unwrap();
            let y = Numeric::from_str("55").unwrap();
            assert_eq!((x + y).to_string(), "Simplex`Integer[110]".to_string());
        }

        #[test]
        fn it_computes_add_real_real() {
            let x = Numeric::from_str("55.55").unwrap();
            let y = Numeric::from_str("55.55").unwrap();
            assert_eq!((x + y).to_string(), "Simplex`Real[111.1]".to_string());
        }

        #[test]
        fn it_computes_add_int_real() {
            let x = Numeric::from_str("55").unwrap();
            let y = Numeric::from_str("55.55").unwrap();
            assert_eq!((x + y).to_string(), "Simplex`Real[110.55]".to_string());
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from_str("55.10").unwrap();
            let y = Numeric::from_str("10").unwrap();
            assert_eq!((x + y).to_string(), "Simplex`Real[65.1]".to_string());
        }
    }

    mod low_sized_subtraction {
    }
}
