#[allow(dead_code)]
pub enum StringNotationPattern {
    First,
    Last,
    External,
    Internal,
    Contains
}

#[allow(dead_code)]
pub fn has_notation_character( snp: StringNotationPattern, c: char, s: &str) -> bool {
    match snp {
        StringNotationPattern::First => {
            match s.chars().nth(0) {
                Some(ch) => {
                    if ch == c {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            }
        }, StringNotationPattern::Last => {
            match s.chars().nth(s.len() - 1) {
                Some(ch) => {
                    if ch == c {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            }
        }, StringNotationPattern::External => {
            match (s.chars().nth(s.len() - 1), s.chars().nth(0)) {
                (Some(cha), Some(chb)) => {
                    cha == c || chb == c
                },  (None, Some(cha)) => {
                    cha == c
                }, (Some(cha), None) => {
                    cha == c
                }, _ => false
            }
        }, StringNotationPattern::Internal => {
            let mut num = 0;
            let mut flag = false;
            for i in s.chars() {
                if num != 0 && num != s.len() - 1 {
                    if i == c {
                        flag = true
                    } 
                }
                num = num + 1;
            }
            flag
        }, StringNotationPattern::Contains => {
            let mut flag = false;
            for i in s.chars() {
                if i == c {
                    flag = true;
                }
            }
            flag
        }
    }
}

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

