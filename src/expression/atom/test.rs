#[cfg(test)]
mod tests {
    mod test_intrinsics {
        use expression::atom::structure::SimplexAtom;
        use expression::traits::BaseExpression;

        extern crate num;

        #[test]
        fn it_shows_empty_expression_on_rest() {
            let s_atom = SimplexAtom::from(1.21);
            let x = s_atom.get_rest();
            assert_eq!(s_atom.as_str(), "1.21");
            assert_eq!(x, None);
        }

        #[test]
        fn it_shows_recursive_empty_expression_on_rest() {
            let s_atom = SimplexAtom::from(1.21);
            assert_eq!(s_atom.as_str(), "1.21");

            let x = s_atom.get_rest();
            assert_eq!(x, None);

            let y = s_atom.get_rest();
            assert_eq!(y, None);
        }
    }

    mod test_basic_numbers {
        use expression::atom::structure::SimplexAtom;
        use expression::traits::BaseExpression;

        #[test]
        fn it_instantiates_decimal() {
            let s_atom = SimplexAtom::from(1.21);
            assert_eq!(s_atom.as_str(), "1.21");
        }


        #[test]
        fn it_instantiates_integer_simplify() {
            let s_atom = SimplexAtom::from(1.00);
            assert_eq!(s_atom.as_str(), "1");
        }


        #[test]
        fn it_instantiates_integer() {
            let s_atom = SimplexAtom::from(32);
            assert_eq!(s_atom.as_str(), "32");
        }
    }
}
