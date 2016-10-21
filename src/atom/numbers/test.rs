#[cfg(test)]
mod tests {
    mod parsing_tests {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_parses_int() {
            let x = Numeric::from("55");
            assert_eq!(x, Numeric::LittleInteger(55));
        }

        #[test]
        fn it_doesnt_parse_bad_int() {
            let x = Numeric::from("dsajfdksjk");
            assert_eq!(x, Numeric::NaN);
        }

        #[test]
        fn it_says_NaN() {
            let x = Numeric::from("dsajfdksjk");
            assert_eq!(x, Numeric::NaN);
        }

        #[test]
        fn it_parses_real() {
            let x = Numeric::from("55.552");
            assert_eq!(x.simplify().as_str(), "55.552");
        }

        #[test]
        fn it_doesnt_parse_bad_real() {
            let x = Numeric::from("dsajfd.ksjk");
            assert_eq!(x, Numeric::NaN);
        }
    }

    mod formatting_tests {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_shows_int() {
            let x = Numeric::from("55");
            assert_eq!(x.as_str(), "55");
        }

        #[test]
        fn it_shows_real() {
            let x = Numeric::from("55.55");
            assert_eq!(x.as_str(), "55.55");
        }
    }

    mod low_sized_addition {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_add_int_int() {
            let x = Numeric::from("55");
            let y = Numeric::from("55");
            assert_eq!((x + y).as_str(), "110");
        }

        #[test]
        fn it_computes_add_real_real() {
            let x = Numeric::from("55.55");
            let y = Numeric::from("55.55");
            assert_eq!((x + y).as_str(), "111.10");
        }

        #[test]
        fn it_computes_add_int_real() {
            let x = Numeric::from("55");
            let y = Numeric::from("55.55");
            assert_eq!((x + y).as_str(), "110.55");
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from("55.10");
            let y = Numeric::from("10");
            assert_eq!((x + y).as_str(), "65.10");
        }
    }

    mod low_sized_subtraction {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_add_int_int() {
            let x = Numeric::from("55");
            let y = Numeric::from("55");
            assert_eq!((x - y).as_str(), "0");
        }

        #[test]
        fn it_computes_add_real_real() {
            let x = Numeric::from("55.55");
            let y = Numeric::from("45.55");
            assert_eq!((x - y).as_str(), "10");
        }

        #[test]
        fn it_computes_add_int_real() {
            let x = Numeric::from("55");
            let y = Numeric::from("55.55");
            assert_eq!((x - y).as_str(), "-0.55");
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from("55.10");
            let y = Numeric::from("10");
            assert_eq!((x - y).as_str(), "45.10");
        }
    }

    mod low_sized_multiplication {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_mul_int_int() {
            let x = Numeric::from("50");
            let y = Numeric::from("50");
            assert_eq!((x * y).as_str(), "2500");
        }

        #[test]
        fn it_computes_mul_real_real() {
            let x = Numeric::from("0.5");
            let y = Numeric::from("0.5");
            assert_eq!((x * y).as_str(), "0.25");
        }

        #[test]
        fn it_computes_mul_int_real() {
            let x = Numeric::from("50");
            let y = Numeric::from(".5");
            assert_eq!((x * y).as_str(), "25");
        }

        #[test]
        fn it_computes_mul_real_int() {
            let x = Numeric::from("55.10");
            let y = Numeric::from("10");
            assert_eq!((x * y).as_str(), "551");
        }
    }

    mod low_sized_division {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_mul_int_int() {
            let x = Numeric::from("50");
            let y = Numeric::from("50");
            assert_eq!((x / y).as_str(), "1");
        }

        #[test]
        fn it_computes_mul_real_real() {
            let x = Numeric::from("4.4");
            let y = Numeric::from("2.2");
            assert_eq!((x / y).as_str(), "2");
        }

        #[test]
        fn it_computes_mul_int_real() {
            let x = Numeric::from("60");
            let y = Numeric::from("2.5");
            assert_eq!((x / y).as_str(), "24");
        }

        #[test]
        fn it_computes_add_real_int() {
            let x = Numeric::from("55.10");
            let y = Numeric::from("5");
            assert_eq!((x / y).as_str(), "11.02");
        }
    }

    mod low_sized_equality {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_eq_int_int() {
            let x = Numeric::from("50");
            let y = Numeric::from("50");
            assert_eq!((x == y), true);
        }

        #[test]
        fn it_computes_eq_real_real() {
            let x = Numeric::from("4.4");
            let y = Numeric::from("4.4");
            assert_eq!((x == y), true);
        }

        #[test]
        fn it_computes_eq_int_real() {
            let x = Numeric::from("60");
            let y = Numeric::from("60.0");
            assert_eq!((x == y), true);
        }

        #[test]
        fn it_computes_eq_real_int() {
            let x = Numeric::from("5.0");
            let y = Numeric::from("5");
            assert_eq!((x == y), true);
        }
    }

    mod low_sized_inequality {
        use atom::numbers::number::Numeric;
        #[test]
        fn it_computes_eq_int_int() {
            let x = Numeric::from("50");
            let y = Numeric::from("51");
            assert_eq!((x != y), true);
        }

        #[test]
        fn it_computes_eq_real_real() {
            let x = Numeric::from("4.4");
            let y = Numeric::from("2.4");
            assert_eq!((x != y), true);
        }

        #[test]
        fn it_computes_eq_int_real() {
            let x = Numeric::from("60");
            let y = Numeric::from("20.0");
            assert_eq!((x != y), true);
        }

        #[test]
        fn it_computes_eq_real_int() {
            let x = Numeric::from("5.0");
            let y = Numeric::from("2");
            assert_eq!((x != y), true);
        }
    }

    mod larger_sized_tests {
        use atom::numbers::number::Numeric;

        #[test]
        fn it_computes_big_mul_real_real() {
            let x = Numeric::from("25.5");
            let y = Numeric::from("200.0");
            assert_eq!((x * y).as_str(), "5100");
        }

        #[test]
        fn it_doesnt_cast_gt_ten_lhs() {
            let x = Numeric::from("210000000000.0");
            let y = Numeric::from("2000000000000000000000000000.0");
            let z = Numeric::from("2000000000000000000000000000.0");
            let a = x * y;
            let b = a / z;
            assert_eq!((b).as_str(), "2.10000E+11");
        }

        #[test]
        fn it_casts_lte_ten_lhs() {
            let x = Numeric::from("2100000000.0");
            let y = Numeric::from("2000000000000000000000000000.0");
            let z = Numeric::from("2000000000000000000000000000.0");
            let a = x * y;
            let b = a / z;
            assert_eq!((b).as_str(), "2100000000");
        }
    }
}
