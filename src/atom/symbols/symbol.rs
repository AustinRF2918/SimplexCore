use parsing::utilities::symbols::representable_symbol;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Symbol {
    name: String,
    // parent: SymbolTableDispatchee
}

impl Symbol {
    pub fn from_str(n: &str) -> Option<Symbol> {
        if representable_symbol(n) {
            Some(Symbol {
                name: n.to_string(),
            })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> &String {
        &self.name
    }
}
