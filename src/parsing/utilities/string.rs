use regex::{Regex, RegexBuilder};

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
            let f_regex: Regex = Regex::new(("^".to_string() + c.to_string().as_str() + ".*$").as_str()).unwrap();
            let captures = f_regex.captures(s);
            match captures {
                Some(c) => true,
                None => false
            }
        }, StringNotationPattern::Last => {
            let l_regex: Regex = Regex::new(("^".to_string() + ".*" + c.to_string().as_str() + "$").as_str()).unwrap();
            let captures = l_regex.captures(s);
            match captures {
                Some(c) => true,
                None => false
            }
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
            let l_regex: Regex = Regex::new(("^".to_string() + ".*" + c.to_string().as_str() + ".*" + "$").as_str()).unwrap();
            let captures = l_regex.captures(s);
            match captures {
                Some(c) => true,
                None => false
            }
        }
    }
}

// TODO: Make tests
pub fn representable_string(s: &str) -> bool {
    let mut begin_flag = false;
    let mut end_flag = false;
    let mut error_flag = false;

    for letter in s.chars() {
        if !begin_flag {
            if letter == '"' {
                begin_flag = true;
            } else if letter != ' ' {
                error_flag = true;
            }
        } else if !end_flag {
            if letter == '"' {
                end_flag = true;
            } 
        } else {
            if letter != ' ' {
                error_flag = true;
            }
        }
    } 

    !error_flag
}
