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
    }
}
