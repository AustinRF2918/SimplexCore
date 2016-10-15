#[cfg(test)]
mod tests {
    mod first_tests {
        use parsing::utilities::string::has_notation_character;
        use parsing::utilities::string::StringNotationPattern;

        #[test]
        fn it_works_with_tic_not_first() {
            assert_eq!(false, has_notation_character( StringNotationPattern::First, '`', "Hello"));
        }

        #[test]
        fn it_works_with_tic_first_and_last() {
            assert_eq!(true, has_notation_character( StringNotationPattern::First, '`', "`Hello`"));
        }

        #[test]
        fn it_doesnt_works_with_tic_and_last() {
            assert_eq!(false, has_notation_character( StringNotationPattern::First, '`', "Hello`"));
        }

        #[test]
        fn it_doesnt_works_with_tic_middle() {
            assert_eq!(false, has_notation_character( StringNotationPattern::First, '`', "He`llo"));
        }
    }

    mod last_tests {
        use parsing::utilities::string::has_notation_character;
        use parsing::utilities::string::StringNotationPattern;

        #[test]
        fn it_works_with_tic_not_first() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Last, '`', "Hello"));
        }

        #[test]
        fn it_works_with_tic_first_and_last() {
            assert_eq!(true, has_notation_character( StringNotationPattern::Last, '`', "`Hello`"));
        }

        #[test]
        fn it_doesnt_work_with_tic_and_first() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Last, '`', "`Hello"));
        }

        #[test]
        fn it_doesnt_work_with_tic_middle() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Last, '`', "He`llo"));
        }
    }

    mod internal_tests {
        use parsing::utilities::string::has_notation_character;
        use parsing::utilities::string::StringNotationPattern;

        #[test]
        fn it_works_with_tic_not_first() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Internal, '`', "Hello"));
        }

        #[test]
        fn it_doesnt_work_with_tic_first_and_last() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Internal, '`', "`Hello`"));
        }

        #[test]
        fn it_doesnt_work_with_tic_first() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Internal, '`', "`Hello"));
        }

        #[test]
        fn it_works_with_tic_middle() {
            assert_eq!(true, has_notation_character( StringNotationPattern::Internal, '`', "He`llo"));
        }
    }

    mod external_tests {
        use parsing::utilities::string::has_notation_character;
        use parsing::utilities::string::StringNotationPattern;

        #[test]
        fn it_works_with_tic_first() {
            assert_eq!(true, has_notation_character( StringNotationPattern::External, '`', "`Hello"));
        }

        #[test]
        fn it_doesnt_work_with_tic_not_first() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Internal, '`', "Hello"));
        }

        #[test]
        fn it_works_with_tic_first_and_last() {
            assert_eq!(true, has_notation_character( StringNotationPattern::External, '`', "`Hello`"));
        }

        #[test]
        fn it_doesnt_work_with_tic_middle() {
            assert_eq!(false, has_notation_character( StringNotationPattern::External, '`', "He`llo"));
        }
    }

    mod contains_tests {
        use parsing::utilities::string::has_notation_character;
        use parsing::utilities::string::StringNotationPattern;

        #[test]
        fn it_works_with_tic_first() {
            assert_eq!(true, has_notation_character( StringNotationPattern::Contains, '`', "`Hello"));
        }

        #[test]
        fn it_works_with_no_tic() {
            assert_eq!(false, has_notation_character( StringNotationPattern::Contains, '`', "Hello"));
        }

        #[test]
        fn it_works_with_tic_first_and_last() {
            assert_eq!(true, has_notation_character( StringNotationPattern::Contains, '`', "`Hello`"));
        }

        #[test]
        fn it_doesnt_work_with_tic_middle() {
            assert_eq!(true, has_notation_character( StringNotationPattern::Contains, '`', "He`llo"));
        }
    }
}
