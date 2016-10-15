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
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() + rhs)
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs + d128::from_str(rhs.to_string().as_str()).unwrap())
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs + rhs)
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
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() - rhs)
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs - d128::from_str(rhs.to_string().as_str()).unwrap())
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs - rhs)
            }
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs * rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() * rhs)
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs * d128::from_str(rhs.to_string().as_str()).unwrap())
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs * rhs)
            }
        }
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                //Unsafe, very very unsafe.
                Numeric::LittleReal(d128::from_str((lhs as f64 / rhs as f64).to_string().as_str()).unwrap())
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() / rhs)
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs / d128::from_str(rhs.to_string().as_str()).unwrap())
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs / rhs)
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
            assert_eq!(x.is_some(), true);
            assert_eq!(x.unwrap().to_string(), "Simplex`Real[NaN]");
        }

        #[test]
        fn it_parses_real() {
            let x = Numeric::from_str("55.552").unwrap();
            assert_eq!(x.head(), "Simplex`Real".to_string());
        }

        #[test]
        fn it_doesnt_parse_bad_real() {
            let x = Numeric::from_str("dsajfd.ksjk");
            assert_eq!(x.is_some(), true);
            assert_eq!(x.unwrap().to_string(), "Simplex`Real[NaN]");
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
            assert_eq!((x + y).to_string(), "Simplex`Real[111.10]".to_string());
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
            assert_eq!((x + y).to_string(), "Simplex`Real[65.10]".to_string());
        }
    }

    mod low_sized_subtraction {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_add_int_int() {
            let x = Numeric::from_str("55").unwrap();
            let y = Numeric::from_str("55").unwrap();
            assert_eq!((x - y).to_string(), "Simplex`Integer[0]".to_string());
        }

        #[test]
        fn it_computes_add_real_real() {
            let x = Numeric::from_str("55.55").unwrap();
            let y = Numeric::from_str("45.55").unwrap();
            assert_eq!((x - y).to_string(), "Simplex`Real[10.00]".to_string());
        }

        #[test]
        fn it_computes_add_int_real() {
            let x = Numeric::from_str("55").unwrap();
            let y = Numeric::from_str("55.55").unwrap();
            assert_eq!((x - y).to_string(), "Simplex`Real[-0.55]".to_string());
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from_str("55.10").unwrap();
            let y = Numeric::from_str("10").unwrap();
            assert_eq!((x - y).to_string(), "Simplex`Real[45.10]".to_string());
        }
    }

    mod low_sized_multiplication {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_mul_int_int() {
            let x = Numeric::from_str("50").unwrap();
            let y = Numeric::from_str("50").unwrap();
            assert_eq!((x * y).to_string(), "Simplex`Integer[2500]".to_string());
        }

        #[test]
        fn it_computes_mul_real_real() {
            let x = Numeric::from_str("0.5").unwrap();
            let y = Numeric::from_str("0.5").unwrap();
            assert_eq!((x * y).to_string(), "Simplex`Real[0.25]".to_string());
        }

        #[test]
        fn it_computes_mul_int_real() {
            let x = Numeric::from_str("50").unwrap();
            let y = Numeric::from_str(".5").unwrap();
            assert_eq!((x * y).to_string(), "Simplex`Real[25.0]".to_string());
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from_str("55.10").unwrap();
            let y = Numeric::from_str("10").unwrap();
            assert_eq!((x * y).to_string(), "Simplex`Real[551.00]".to_string());
        }
    }

    mod low_sized_division {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_mul_int_int() {
            let x = Numeric::from_str("50").unwrap();
            let y = Numeric::from_str("50").unwrap();
            assert_eq!((x / y).to_string(), "Simplex`Real[1]".to_string());
        }

        #[test]
        fn it_computes_mul_real_real() {
            let x = Numeric::from_str("4.4").unwrap();
            let y = Numeric::from_str("4").unwrap();
            assert_eq!((x / y).to_string(), "Simplex`Real[1.1]".to_string());
        }

        #[test]
        fn it_computes_mul_int_real() {
            let x = Numeric::from_str("60").unwrap();
            let y = Numeric::from_str("2.5").unwrap();
            assert_eq!((x / y).to_string(), "Simplex`Real[24]".to_string());
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from_str("55.10").unwrap();
            let y = Numeric::from_str("5").unwrap();
            assert_eq!((x / y).to_string(), "Simplex`Real[11.02]".to_string());
        }
    }
}
