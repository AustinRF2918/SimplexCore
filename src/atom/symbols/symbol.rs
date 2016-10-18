pub struct Symbol {
    name: String,
    namespace: String
    // parent: SymbolTableDispatchee
}

impl Symbol {
    pub fn new(n: &str) -> Symbol {
        Symbol {
            name: n.to_string(),
            namespace: "Symbol::Default".to_string()
        }
    }
}
