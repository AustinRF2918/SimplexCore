#[cfg(test)]
mod tests {
    mod parsing_tests {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_parses_int() {
            let x = Numeric::from_str("55").unwrap();
            assert_eq!(x.head(), "Simplex`Integer");
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
            assert_eq!(x.head(), "Simplex`Real");
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
