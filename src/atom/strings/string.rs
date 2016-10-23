use parsing::utilities::string::representable_string;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SString {
    pub contents: String,
}

impl SString {
    #[allow(dead_code)]
    pub fn from_str(n: &str) -> Option<SString> {
        if representable_string(n) {
            Some(SString { contents: n.to_string() })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("\"{}\"", self.contents)
    }
}
