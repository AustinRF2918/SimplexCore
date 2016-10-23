#[cfg(test)]
mod tests {
    mod string_tests {
        mod first_tests {
            use parsing::utilities::string::representable_string;

            #[test]
            fn it_fails_with_only_a_quote() {
                assert_eq!(false, representable_string("\"Hello"));
            }

            #[test]
            fn it_succeeds_with_two_quotes() {
                assert_eq!(true, representable_string("\"Hello\""));
            }

            #[test]
            fn it_fails_with_only_end_quote() {
                assert_eq!(false, representable_string("Hello\""));
            }

            #[test]
            fn it_fails_with_quotes_in_middle() {
                assert_eq!(false, representable_string("H\"el\"lo"));
            }

            #[test]
            fn it_fails_a_quote_in_middle() {
                assert_eq!(false, representable_string("H\"llo"));
            }

            #[test]
            fn it_fails_when_only_quote() {
                assert_eq!(false, representable_string("\""));
            }

            #[test]
            fn it_fails_on_null() {
                assert_eq!(false, representable_string(""));
            }

            #[test]
            fn it_succeeds_on_null_string() {
                assert_eq!(true, representable_string("\"\""));
            }
        }

        mod last_tests {
            use parsing::utilities::string::has_notation_character;
            use parsing::utilities::string::StringNotationPattern;

            #[test]
            fn it_works_with_tic_not_first() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Last, '`', "Hello"));
            }

            #[test]
            fn it_works_with_tic_first_and_last() {
                assert_eq!(true,
                           has_notation_character(StringNotationPattern::Last, '`', "`Hello`"));
            }

            #[test]
            fn it_doesnt_work_with_tic_and_first() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Last, '`', "`Hello"));
            }

            #[test]
            fn it_doesnt_work_with_tic_middle() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Last, '`', "He`llo"));
            }
        }

        mod internal_tests {
            use parsing::utilities::string::has_notation_character;
            use parsing::utilities::string::StringNotationPattern;

            #[test]
            fn it_works_with_tic_not_first() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Internal, '`', "Hello"));
            }

            #[test]
            fn it_doesnt_work_with_tic_first_and_last() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Internal, '`', "`Hello`"));
            }

            #[test]
            fn it_doesnt_work_with_tic_first() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Internal, '`', "`Hello"));
            }

            #[test]
            fn it_works_with_tic_middle() {
                assert_eq!(true,
                           has_notation_character(StringNotationPattern::Internal, '`', "He`llo"));
            }
        }

        mod external_tests {
            use parsing::utilities::string::has_notation_character;
            use parsing::utilities::string::StringNotationPattern;

            #[test]
            fn it_works_with_tic_first() {
                assert_eq!(true,
                           has_notation_character(StringNotationPattern::External, '`', "`Hello"));
            }

            #[test]
            fn it_doesnt_work_with_tic_not_first() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Internal, '`', "Hello"));
            }

            #[test]
            fn it_works_with_tic_first_and_last() {
                assert_eq!(true,
                           has_notation_character(StringNotationPattern::External, '`', "`Hello`"));
            }

            #[test]
            fn it_doesnt_work_with_tic_middle() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::External, '`', "He`llo"));
            }
        }

        mod contains_tests {
            use parsing::utilities::string::has_notation_character;
            use parsing::utilities::string::StringNotationPattern;

            #[test]
            fn it_works_with_tic_first() {
                assert_eq!(true,
                           has_notation_character(StringNotationPattern::Contains, '`', "`Hello"));
            }

            #[test]
            fn it_works_with_no_tic() {
                assert_eq!(false,
                           has_notation_character(StringNotationPattern::Contains, '`', "Hello"));
            }

            #[test]
            fn it_works_with_tic_first_and_last() {
                assert_eq!(true,
                           has_notation_character(StringNotationPattern::Contains, '`', "`Hello`"));
            }

            #[test]
            fn it_doesnt_work_with_tic_middle() {
                assert_eq!(true,
                           has_notation_character(StringNotationPattern::Contains, '`', "He`llo"));
            }
        }
    }
    mod numeric_tests {
        mod integer_parse_tests {
            use parsing::utilities::numerics::representable_integer;
            use parsing::utilities::numerics::get_point_location;
            use parsing::utilities::numerics::get_plus_location;

            #[test]
            fn it_finds_no_point() {
                assert_eq!(None, get_point_location("1"));
            }

            #[test]
            fn it_finds_a_point() {
                assert_eq!(Some(1), get_point_location("1.0"));
            }

            #[test]
            fn it_finds_a_more_complex_point() {
                assert_eq!(Some(6), get_point_location("123456.043143"));
            }

            #[test]
            fn it_finds_a_more_complex_point_where_multiple_points() {
                assert_eq!(Some(6), get_point_location("123456.0431.43"));
            }
            #[test]
            fn it_finds_no_plus() {
                assert_eq!(None, get_point_location("1"));
            }

            #[test]
            fn it_finds_a_plus() {
                assert_eq!(Some(1), get_plus_location("1+0"));
            }

            #[test]
            fn it_finds_a_more_complex_plus() {
                assert_eq!(Some(6), get_plus_location("123456+043143"));
            }

            #[test]
            fn it_finds_a_more_complex_plus_where_multiple_plus() {
                assert_eq!(Some(6), get_plus_location("123456+0431+43"));
            }

            #[test]
            fn it_works_with_regular_integer() {
                assert_eq!(true, representable_integer("1"));
            }

            #[test]
            fn it_works_with_bigger_integer() {
                assert_eq!(true, representable_integer("10"));
            }

            #[test]
            fn it_works_with_negative_integer() {
                assert_eq!(true, representable_integer("-50"));
            }

            #[test]
            fn it_works_with_larger_negative_integer() {
                assert_eq!(true, representable_integer("-500"));
            }

            #[test]
            fn it_works_with_zero_decimals_0() {
                assert_eq!(true, representable_integer("50.0"));
            }

            #[test]
            fn it_works_with_zero_decimals_000000() {
                assert_eq!(true, representable_integer("50.0000000"));
            }

            #[test]
            fn it_doesnt_work_with_two_negatives() {
                assert_eq!(false, representable_integer("--50"));
            }

            #[test]
            fn it_doesnt_work_with_middle_negatives() {
                assert_eq!(false, representable_integer("5-0"));
            }

            #[test]
            fn it_doesnt_work_with_end_negatives() {
                assert_eq!(false, representable_integer("50-"));
            }

            #[test]
            fn it_doesnt_work_with_decimals_1() {
                assert_eq!(false, representable_integer("50.1"));
            }

            #[test]
            fn it_doesnt_work_with_decimals_01() {
                assert_eq!(false, representable_integer("50.01"));
            }

            #[test]
            fn it_doesnt_work_with_decimals_10() {
                assert_eq!(false, representable_integer("50.10"));
            }
        }

        mod integer_conversion_tests {
            use parsing::utilities::numerics::get_representable_integer;

            #[test]
            fn it_converts_from_regular_integer() {
                assert_eq!(Some(1), get_representable_integer("1"));
            }

            #[test]
            fn it_converts_from_bigger_integer() {
                assert_eq!(Some(-50), get_representable_integer("-50"));
            }

            #[test]
            fn it_converts_from_negative_integer() {
                assert_eq!(Some(-50), get_representable_integer("-50"));
            }

            #[test]
            fn it_converts_from_larger_negative_integer() {
                assert_eq!(Some(-500), get_representable_integer("-500"));
            }

            #[test]
            fn it_converts_from_zero_decimals_0() {
                assert_eq!(Some(50), get_representable_integer("50.0"));
            }

            #[test]
            fn it_converts_from_zero_decimals_000000() {
                assert_eq!(Some(50), get_representable_integer("50.0000000"));
            }

            #[test]
            fn it_converts_from_exponent_notation_0() {
                assert_eq!(Some(1), get_representable_integer("1.00E+0"));
            }

            #[test]
            fn it_converts_from_exponent_notation_1() {
                assert_eq!(Some(10), get_representable_integer("1.00E+1"));
            }

            #[test]
            fn it_converts_from_exponent_notation_2() {
                assert_eq!(Some(100), get_representable_integer("1.00E+2"));
            }

            #[test]
            fn it_converts_from_exponent_notation_3() {
                assert_eq!(Some(1000), get_representable_integer("1.00E+3"));
            }

            #[test]
            fn it_converts_from_exponent_notation_8() {
                assert_eq!(Some(100000000), get_representable_integer("1.00E+8"));
            }

            #[test]
            fn it_converts_from_exponent_notation_9() {
                assert_eq!(Some(1000000000), get_representable_integer("1.00E+9"));
            }

            #[test]
            fn it_doesnt_convert_from_exponent_notation_10() {
                assert_eq!(Some(10000000000), get_representable_integer("1.00E+10"));
            }

            #[test]
            fn it_doesnt_convert_bad_decimal_from_exponent_notation() {
                assert_eq!(None, get_representable_integer("1.0220E+2"));
            }

            #[test]
            fn it_doesnt_convert_from_two_negatives() {
                assert_eq!(None, get_representable_integer("--50"));
            }

            #[test]
            fn it_doesnt_convert_from_middle_negatives() {
                assert_eq!(None, get_representable_integer("5-0"));
            }

            #[test]
            fn it_doesnt_convert_from_end_negatives() {
                assert_eq!(None, get_representable_integer("50-"));
            }

            #[test]
            fn it_doesnt_convert_from_decimals_1() {
                assert_eq!(None, get_representable_integer("50.1"));
            }

            #[test]
            fn it_doesnt_convert_from_decimals_01() {
                assert_eq!(None, get_representable_integer("50.01"));
            }

            #[test]
            fn it_doesnt_convert_from_decimals_10() {
                assert_eq!(None, get_representable_integer("50.10"));
            }
        }
    }
    mod symbol_tests {
        use parsing::utilities::symbols::representable_symbol;

        #[test]
        fn it_succeeds_on_a_simple_symbol() {
            assert_eq!(true, representable_symbol("x"));
            assert_eq!(true, representable_symbol("hello"));
            assert_eq!(true, representable_symbol("HelloWorld"));
            assert_eq!(true, representable_symbol("HelloWorldMe"));
            assert_eq!(true, representable_symbol("ABCDEFGHI"));
        }

        #[test]
        fn it_fails_on_a_numeric() {
            assert_eq!(false, representable_symbol("0"));
            assert_eq!(false, representable_symbol("2"));
            assert_eq!(false, representable_symbol("22"));
            assert_eq!(false, representable_symbol("22a"));
            assert_eq!(false, representable_symbol("22a2a"));
            assert_eq!(false, representable_symbol("22a2a1"));
        }

        #[test]
        fn it_succeeds_on_characters_then_num() {
            assert_eq!(true, representable_symbol("a2"));
            assert_eq!(true, representable_symbol("ab2"));
            assert_eq!(true, representable_symbol("BabZ2"));
            assert_eq!(true, representable_symbol("BabZ2242"));
        }

        #[test]
        fn it_fails_with_operators() {
            assert_eq!(false, representable_symbol("a+2"));
            assert_eq!(false, representable_symbol("ab-2"));
            assert_eq!(false, representable_symbol("B*abZ2"));
            assert_eq!(false, representable_symbol("BabZ/2242"));
            assert_eq!(false, representable_symbol("Babl==242"));
        }
    }
}
