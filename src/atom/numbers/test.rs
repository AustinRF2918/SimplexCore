#[cfg(test)]
mod tests {
    mod parsing_tests {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_parses_int() {
            let x = Numeric::from_str("55");
            assert_eq!(x.head(), "Simplex`Integer");
        }

        #[test]
        fn it_doesnt_parse_bad_int() {
            let x = Numeric::from_str("dsajfdksjk");
            assert_eq!(x.simplify().to_string(), "NaN");
        }

        #[test]
        fn it_says_NaN() {
            let x = Numeric::from_str("dsajfdksjk");
            assert_eq!(x, Numeric::NaN);
        }

        #[test]
        fn it_parses_real() {
            let x = Numeric::from_str("55.552");
            assert_eq!(x.head(), "Simplex`Real");
        }

        #[test]
        fn it_doesnt_parse_bad_real() {
            let x = Numeric::from_str("dsajfd.ksjk");
            assert_eq!(x.to_string(), "NaN");
        }
    }

    mod formatting_tests {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_shows_int() {
            let x = Numeric::from_str("55");
            assert_eq!(x.to_string(), "55".to_string());
        }

        #[test]
        fn it_shows_real() {
            let x = Numeric::from_str("55.55");
            assert_eq!(x.to_string(), "55.55".to_string());
        }
    }

    mod low_sized_addition {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_add_int_int() {
            let x = Numeric::from_str("55");
            let y = Numeric::from_str("55");
            assert_eq!((x + y).to_string(), "110".to_string());
        }

        #[test]
        fn it_computes_add_real_real() {
            let x = Numeric::from_str("55.55");
            let y = Numeric::from_str("55.55");
            assert_eq!((x + y).to_string(), "111.10".to_string());
        }

        #[test]
        fn it_computes_add_int_real() {
            let x = Numeric::from_str("55");
            let y = Numeric::from_str("55.55");
            assert_eq!((x + y).to_string(), "110.55".to_string());
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from_str("55.10");
            let y = Numeric::from_str("10");
            assert_eq!((x + y).to_string(), "65.10".to_string());
        }
    }

    mod low_sized_subtraction {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_add_int_int() {
            let x = Numeric::from_str("55");
            let y = Numeric::from_str("55");
            assert_eq!((x - y).to_string(), "0".to_string());
        }

        #[test]
        fn it_computes_add_real_real() {
            let x = Numeric::from_str("55.55");
            let y = Numeric::from_str("45.55");
            assert_eq!((x - y).to_string(), "10.00".to_string());
        }

        #[test]
        fn it_computes_add_int_real() {
            let x = Numeric::from_str("55");
            let y = Numeric::from_str("55.55");
            assert_eq!((x - y).to_string(), "-0.55".to_string());
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from_str("55.10");
            let y = Numeric::from_str("10");
            assert_eq!((x - y).to_string(), "45.10".to_string());
        }
    }

    mod low_sized_multiplication {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_mul_int_int() {
            let x = Numeric::from_str("50");
            let y = Numeric::from_str("50");
            assert_eq!((x * y).to_string(), "2500".to_string());
        }

        #[test]
        fn it_computes_mul_real_real() {
            let x = Numeric::from_str("0.5");
            let y = Numeric::from_str("0.5");
            assert_eq!((x * y).to_string(), "0.25".to_string());
        }

        #[test]
        fn it_computes_mul_int_real() {
            let x = Numeric::from_str("50");
            let y = Numeric::from_str(".5");
            assert_eq!((x * y).to_string(), "25".to_string());
        }

        #[test]
        fn it_computes_mul_real_int() {
            let x = Numeric::from_str("55.10");
            let y = Numeric::from_str("10");
            assert_eq!((x * y).to_string(), "551".to_string());
        }
    }

    mod low_sized_division {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_mul_int_int() {
            let x = Numeric::from_str("50");
            let y = Numeric::from_str("50");
            assert_eq!((x / y).to_string(), "1".to_string());
        }

        #[test]
        fn it_computes_mul_real_real() {
            let x = Numeric::from_str("4.4");
            let y = Numeric::from_str("2.2");
            assert_eq!((x / y).to_string(), "2".to_string());
        }

        #[test]
        fn it_computes_mul_int_real() {
            let x = Numeric::from_str("60");
            let y = Numeric::from_str("2.5");
            assert_eq!((x / y).to_string(), "24".to_string());
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from_str("55.10");
            let y = Numeric::from_str("5");
            assert_eq!((x / y).to_string(), "11.02".to_string());
        }
    }

    mod larger_sized_tests {
        use atom::numbers::number::Numeric;

        #[test]
        fn it_computes_big_mul_real_real() {
            let x = Numeric::from_str("25.5");
            let y = Numeric::from_str("200.0");
            assert_eq!((x * y).to_string(), "5100.00".to_string());
        }
    }
}
