pub mod checks {
    use parsing::utilities::string::has_notation_character;
    use parsing::utilities::string::StringNotationPattern;

    #[allow(dead_code)]
    pub fn fully_qualified_symbol_name(name: &str) -> bool {
        !(has_notation_character(StringNotationPattern::First, '`', name)) &&
        !(has_notation_character(StringNotationPattern::Last, '`', name)) &&
        (has_notation_character(StringNotationPattern::Contains, '`', name)) &&
        !(name.contains("``"))
    }

    #[allow(dead_code)]
    pub fn valid_context_name(context: &str) -> bool {
        has_notation_character(StringNotationPattern::Last, '`', context) &&
        !(has_notation_character(StringNotationPattern::First, '`', context)) &&
        !context.contains("``")
    }

    #[allow(dead_code)]
    pub fn valid_context_name_initial_bq(context: &str) -> bool {
        has_notation_character(StringNotationPattern::Last, '`', context) && !context.contains("``")
    }

    pub fn ensure_context(name: &str) -> String {
        if name.len() == 0 {
            "Error`Undefined".to_string()
        } else if has_notation_character(StringNotationPattern::Contains, '`', name) {
            if fully_qualified_symbol_name(name) {
                name.to_string()
            } else {
                "Error`Malformed".to_string()
            }
        } else {
            "Simplex`".to_string() + name
        }
    }

    #[allow(dead_code)]
    pub fn strip_context(name: &str) -> String {
        if has_notation_character(StringNotationPattern::Contains, '`', name) {
            let mut stripped_context = String::new();
            let mut flag = false;

            for letter in name.chars() {
                if flag {
                    stripped_context.push(letter);
                } else if letter == '`' {
                    flag = true;
                }
            }
            stripped_context
        } else {
            name.to_string()
        }
    }
}

pub mod applications {
    use expression::structures::integrity::checks::ensure_context;

    #[allow(dead_code)]
    pub fn vec_to_symbols(symbols: Vec<String>) -> Vec<String> {
        symbols.into_iter().map(|s| ensure_context(&s)).collect()
    }

    // Implement once symbols are implemented.
    //
    // pub fn map_to_symbols(hash: HashMap<String, symbol::structures::core_symbol> ) -> HashkMap<String, symbol::structures::core_symbol> {
    // }
    //

}

#[cfg(test)]
mod tests {
    mod fully_qualified_tests {
        use expression::structures::integrity::checks::fully_qualified_symbol_name;

        #[test]
        fn it_denies_fully_qualified_left_mark() {
            assert_eq!(false, fully_qualified_symbol_name("`Hello"));
        }

        #[test]
        fn it_denies_fully_qualified_right_mark() {
            assert_eq!(false, fully_qualified_symbol_name("Hello`"));
        }

        #[test]
        fn it_denies_fully_qualified_both_mark() {
            assert_eq!(false, fully_qualified_symbol_name("`Hello`"));
        }

        #[test]
        fn it_denies_fully_qualified_multiple_mark() {
            assert_eq!(false, fully_qualified_symbol_name("`He`llo`"));
        }

        #[test]
        fn it_denies_fully_qualified_two_mark() {
            assert_eq!(false, fully_qualified_symbol_name("He``llo"));
        }

        #[test]
        fn it_denies_fully_qualified_multiple_and_two_mark() {
            assert_eq!(false, fully_qualified_symbol_name("`He``llo`"));
        }

        #[test]
        fn it_accepts_good_name() {
            assert_eq!(true, fully_qualified_symbol_name("Simplex`Hello"));
        }
    }

    mod valid_context_tests {
        use expression::structures::integrity::checks::{valid_context_name,
                                                        valid_context_name_initial_bq};

        #[test]
        fn it_denies_bad_context_name() {
            assert_eq!(false, valid_context_name("Simplex"));
        }

        #[test]
        fn it_denies_bad_with_double_tic_context_name() {
            assert_eq!(false, valid_context_name("Sy``stem`"));
        }

        #[test]
        fn it_accepts_good_context_name() {
            assert_eq!(true, valid_context_name("Simplex`"));
        }

        #[test]
        fn it_denies_tic_front_when_not_toggled() {
            assert_eq!(false, valid_context_name("`Simplex`"));
        }

        #[test]
        fn it_accepts_tic_front_when_toggled() {
            assert_eq!(true, valid_context_name_initial_bq("`Simplex`"));
        }

        #[test]
        fn it_accepts_tic_not_front_when_toggled() {
            assert_eq!(true, valid_context_name_initial_bq("Simplex`"));
        }

        #[test]
        fn it_rejects_weird_double_when_toggled() {
            assert_eq!(false, valid_context_name_initial_bq("Sys``tem`"));
        }

        #[test]
        fn it_rejects_extra_when_toggled() {
            assert_eq!(true, valid_context_name_initial_bq("Sys`tem`"));
        }
    }

    mod ensure_context_tests {
        use expression::structures::integrity::checks::ensure_context;

        #[test]
        fn it_denies_bad_context_name() {
            assert_eq!("Simplex`Test", ensure_context("Test"));
        }
    }
}
