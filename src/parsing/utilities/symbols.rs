use regex::Regex;

pub fn representable_symbol(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z]+[0-9a-zA-Z]*$").unwrap();
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
