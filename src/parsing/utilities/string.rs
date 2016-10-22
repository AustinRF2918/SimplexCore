use regex::{Regex, RegexBuilder, Captures};

pub enum StringNotationPattern {
    First,
    Last,
    External,
    Internal,
    Contains,
}

pub fn has_notation_character(snp: StringNotationPattern, c: char, s: &str) -> bool {
    match snp {
        StringNotationPattern::First => {
            let first_regex = Regex::new((format!("^{}.*$", c)).as_str());
            match first_regex {
                Ok(parser) => parser.is_match(s),
                Err(e) => {
                    panic!("The character you passed to has_notation_character( ... ) caused the \
                            regex compiler to crash!");
                }
            }
        }

        StringNotationPattern::Last => {
            let last_regex = Regex::new((format!("^.*{}$", c)).as_str());
            match last_regex {
                Ok(parser) => parser.is_match(s),
                Err(e) => {
                    panic!("The character you passed to has_notation_character( ... ) caused the \
                            regex compiler to crash!");
                }
            }
        }

        StringNotationPattern::External => {
            has_notation_character(StringNotationPattern::First, c, s) ||
            has_notation_character(StringNotationPattern::Last, c, s)
        }

        StringNotationPattern::Internal => {
            for (num, letter) in s.chars().enumerate() {
                if num != 0 && num != s.len() - 1 && letter == c {
                    return true;
                }
            }

            false
        }

        StringNotationPattern::Contains => {
            let contains_regex = Regex::new((format!("^.*{}.*$", c)).as_str());
            match contains_regex {
                Ok(parser) => parser.is_match(s),
                Err(e) => {
                    panic!("The character you passed to has_notation_character( ... ) caused the \
                            regex compiler to crash!");
                }
            }
        }
    }
}

pub fn representable_string(s: &str) -> bool {
    has_notation_character(StringNotationPattern::First, '"', s) &&
    has_notation_character(StringNotationPattern::Last, '"', s) && s.len() > 1
}
