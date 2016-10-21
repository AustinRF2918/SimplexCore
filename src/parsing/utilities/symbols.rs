use regex::Regex;

pub fn representable_symbol(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z]+[`0-9a-zA-Z]*$").unwrap();
    }

    RE.is_match(s)
}
