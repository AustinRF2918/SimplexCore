use regex::Regex;

pub fn representable_integer( s: &str ) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[+-]?[0-9]*(\.?)([0]*)$").unwrap();
    }

    let captures = RE.captures(s);

    match captures {
        Some(c) => {
            let res = c.iter().map(|x| x == Some(s)).fold(false, |acc, i| acc || i);
            res
        }

        None => {
            false
        }
    }
}

pub fn get_representable_integer(s: &str) -> Option<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<digit>[+-]?([0-9]*))(\.?)([0]*)$").unwrap();
    }

    let captures = RE.captures(s);

    match captures {
        Some(c) => {
            Some(c.name("digit").unwrap().parse::<i64>().unwrap())
        }

        None => {
            None
        }
    }
}
