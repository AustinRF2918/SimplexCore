use regex::Regex;

#[allow(dead_code)]
pub fn representable_integer(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[+-]?[0-9]*(\.?)([0]*)$").unwrap();
    }

    RE.is_match(s)
}

// TODO: Fix bug that could occur during dynamic parsing with exponent d128
// numbers.
#[allow(dead_code)]
pub fn representable_numeric(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[+-]?[0-9]*(\.?)([0-9]*)$").unwrap();
    }

    RE.is_match(s)
}

#[allow(dead_code)]
pub fn get_representable_numeric(s: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<numeric>[+-]?[0-9]*(\.?)([0-9]*))$").unwrap();
    }

    if RE.is_match(s) { Some(s) } else { None }
}

#[allow(dead_code)]
pub fn get_representable_integer(s: &str) -> Option<i64> {
    lazy_static! {
        static ref REG_NOTATION: Regex = Regex::new(r"^(?P<digit>[+-]?([0-9]*))(\.?)([0]*)$").unwrap();
        static ref EXP_NOTATION: Regex = Regex::new(r"^(?P<lhs>[+-]?([0-9]*\.[0-9]*))[E](?P<sign>[+-])(?P<rhs>[0-9]+)$").unwrap();
    }

    let captures = REG_NOTATION.captures(s);

    match captures {
        Some(c) => {
            match c.name("digit").unwrap().parse::<i64>() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        }

        None => {
            // Refactor this a bit: We could use closures
            // and unwrap_or_else to make this less "callback hell
            // eque"
            let other_captures = EXP_NOTATION.captures(s);
            match other_captures {
                Some(c) => {
                    let lhs = c.name("lhs").unwrap();
                    let rhs = c.name("rhs").unwrap();
                    let sign = c.name("sign").unwrap();

                    match rhs.parse::<u64>() {
                        Ok(num) => {
                            if sign == "-" {
                                None
                            } else if num > 10 {
                                None
                            } else {
                                let mut s = String::new();
                                let mut curr = num;
                                let mut flag = false;
                                let mut u_flag = false;
                                let mut d_flag = false;

                                for letter in lhs.chars() {
                                    if letter == '.' {
                                        flag = true;
                                    } else {
                                        if !u_flag {
                                            if curr != 0 || !d_flag {
                                                s.push(letter);
                                                d_flag = true;
                                            }
                                        }

                                        if flag {
                                            if curr == 0 {
                                                if letter != '0' {
                                                    u_flag = true;
                                                }
                                            } else {
                                                curr = curr - 1;
                                            }
                                        }
                                    }
                                }

                                if !u_flag {
                                    while curr != 0 {
                                        s.push('0');
                                        curr = curr - 1;
                                    }

                                    match s.parse::<i64>() {
                                        Ok(num) => Some(num),
                                        _ => None,
                                    }
                                } else {
                                    None
                                }
                            }
                        }
                        Err(_) => None,
                    }
                }
                None => None,
            }
        }
    }
}

#[inline]
fn first_location(s: &str, letter: char) -> Option<usize> {
    let mut pos: Option<usize> = None;

    for (num, item) in s.chars().enumerate() {
        if item == letter {
            pos = Some(num);
            break;
        }
    }

    pos
}

#[allow(dead_code)]
pub fn get_point_location(s: &str) -> Option<usize> {
    first_location(s, '.')
}

#[allow(dead_code)]
pub fn get_plus_location(s: &str) -> Option<usize> {
    first_location(s, '+')
}
