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
