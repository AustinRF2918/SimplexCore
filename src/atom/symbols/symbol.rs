use parsing::utilities::symbols::representable_symbol;

pub struct Symbol {
    name: String,
    // parent: SymbolTableDispatchee
}

impl Symbol {
    pub fn new(n: &str) -> Symbol {
        Symbol {
            name: n.to_string(),
        }
    }

    pub fn to_string(&self) -> &String {
        &self.name
    }
}

