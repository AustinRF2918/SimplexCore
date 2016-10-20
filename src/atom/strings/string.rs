use parsing::utilities::string::representable_string;

pub struct SString {
    pub contents: String,
}

impl SString {
    pub fn from_str(n: &str) -> Option<SString> {
        if representable_string(n) {
            Some(SString {
                contents: n.to_string(),
            })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        format!("\"{}\"", self.contents)
    }
}
