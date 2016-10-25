#[cfg(test)]
mod tests {
    mod test_basic_numbers {
        mod test_real_numbers {
            use atom::atom::SimplexAtom;
            use expression::traits::BaseExpression;

            extern crate decimal;
            use decimal::d128;

            extern crate num;
            use std::str::FromStr;

            #[test]
            fn it_instantiates() {
                let s_atom = SimplexAtom::from(1.21);
                assert_eq!(s_atom.get_expression_type(), "Simplex`Atom");
                assert_eq!(s_atom.get_head_name(), "Real");
            }

            #[test]
            fn it_gets_float_value() {
                let s_atom = SimplexAtom::from(100.201);
                assert_eq!(s_atom.get_float_value(),
                           Some(d128::from_str("100.201").unwrap()));
            }

            #[test]
            fn it_doesnt_get_int_value() {
                let s_atom = SimplexAtom::from(100.201);
                assert_eq!(s_atom.get_int_value(), None);
            }

            #[test]
            fn it_doesnt_get_string_value() {
                let s_atom = SimplexAtom::from(100.201);
                assert_eq!(s_atom.get_string_value(), None);
            }
        }

        mod test_int_numbers {
            use atom::atom::SimplexAtom;
            use expression::traits::BaseExpression;

            #[test]
            fn it_instantiates() {
                let s_atom = SimplexAtom::from(1.00);
                assert_eq!(s_atom.get_expression_type(), "Simplex`Atom");
                assert_eq!(s_atom.get_head_name(), "Integer");
            }

            #[test]
            fn it_gets_int_value() {
                let s_atom = SimplexAtom::from(100.000);
                assert_eq!(s_atom.get_int_value(), Some(100));
            }

            #[test]
            fn it_doesnt_get_float_value() {
                let s_atom = SimplexAtom::from(100.000);
                assert_eq!(s_atom.get_float_value(), None);
            }

            #[test]
            fn it_doesnt_get_string_value() {
                let s_atom = SimplexAtom::from(100.00);
                assert_eq!(s_atom.get_string_value(), None);
            }
        }

        mod test_type_deduction {
            use atom::atom::SimplexAtom;
            use expression::traits::BaseExpression;

            #[test]
            fn it_instantiates() {
                let s_atom: SimplexAtom = SimplexAtom::from(32);
                assert_eq!(s_atom.get_expression_type(), "Simplex`Atom");
                assert_eq!(s_atom.get_head_name(), "Integer");
            }
        }

        mod test_symbols {
            use atom::atom::SimplexAtom;
            use expression::traits::BaseExpression;

            #[test]
            fn it_instantiates() {
                let s_atom = SimplexAtom::from("x");
                assert_eq!(s_atom.get_expression_type(), "Simplex`Atom");
                assert_eq!(s_atom.get_head_name(), "Symbol");
            }

            #[test]
            fn it_doesnt_get_int_value() {
                let s_atom = SimplexAtom::from("x");
                assert_eq!(s_atom.get_int_value(), None);
            }

            #[test]
            fn it_doesnt_get_float_value() {
                let s_atom = SimplexAtom::from("Hello");
                assert_eq!(s_atom.get_float_value(), None);
            }

            #[test]
            fn it_doesnt_get_string_value() {
                let s_atom = SimplexAtom::from("World");
                assert_eq!(s_atom.get_string_value(), None);
            }
        }
    }
}
