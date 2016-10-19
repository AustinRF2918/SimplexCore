use regex::{Regex, RegexBuilder, Captures};

#[allow(dead_code)]
pub enum StringNotationPattern {
    First,
    Last,
    External,
    Internal,
    Contains
}

// TODO: Make tests.
fn captures_exist(captures: Option<Captures>) -> bool {
    match captures {
        Some(c) => true,
        None => false
    }
}

#[allow(dead_code)]
pub fn has_notation_character( snp: StringNotationPattern, c: char, s: &str) -> bool {
    match snp {
        StringNotationPattern::First => {
            let f_regex: Regex = Regex::new(("^".to_string() + c.to_string().as_str() + ".*$").as_str()).unwrap();
            captures_exist(f_regex.captures(s))
        },
        StringNotationPattern::Last => {
            let l_regex: Regex = Regex::new(("^".to_string() + ".*" + c.to_string().as_str() + "$").as_str()).unwrap();
            captures_exist(l_regex.captures(s))
        }, StringNotationPattern::External => {
            has_notation_character(StringNotationPattern::First, c, s) ||
            has_notation_character(StringNotationPattern::Last, c, s)
                
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
            let i_regex: Regex = Regex::new(("^".to_string() + ".*" + c.to_string().as_str() + ".*" + "$").as_str()).unwrap();
            captures_exist(i_regex.captures(s))
        }
    }
}

// TODO: Make tests
pub fn representable_string(s: &str) -> bool {
    let f_regex: Regex = Regex::new(("^".to_string() + "\"" + ".*" + "\"" + "$").as_str()).unwrap();
    captures_exist(f_regex.captures(s))
}
