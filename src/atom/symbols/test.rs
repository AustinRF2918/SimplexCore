#[cfg(test)]
mod tests {
    mod general_use {
        use atom::symbols::symbol::Symbol;

        #[test]
        fn it_instantiates() {
            let symbol = Symbol::from_str("x").unwrap();
            assert_eq!(symbol.to_string().as_str(), "x");
        }
    }
}
