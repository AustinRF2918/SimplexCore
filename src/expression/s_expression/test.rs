#[cfg(test)]
mod test {
    mod test_general_functions {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;
        use expression::traits::Transmutable;

        #[test]
        fn it_instantiates() {
            let s_exp = SExpression::new("List");
        }

        #[test]
        fn it_shows_string() {
            let s_exp = SExpression::new("List");
            assert_eq!(s_exp.as_str(), "List[]");
        }

        #[test]
        fn it_pushes_expressions() {
            let s_exp = SExpression::new("List") 
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));

            assert_eq!(s_exp.as_str(), "List[x, y, z]");
        }

        #[test]
        fn it_pushes_numbers() {
            let s_exp = SExpression::new("List") 
                .push_expression(Expression::from("1"))
                .push_expression(Expression::from("2"))
                .push_expression(Expression::from("3"));

            assert_eq!(s_exp.as_str(), "List[1, 2, 3]");
        }
    }

mod test_intrinsics {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::m_expression::structure::MExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_gets_rest() {
            let m_exp = SExpression::new("List")
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));
            assert_eq!(m_exp.get_rest().unwrap().as_str(), "List[x, y, z]");
        }

        #[test]
        fn it_gets_rest_recursively_once() {
            let m_exp = SExpression::new("List")
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));

            let x  = m_exp.get_rest().unwrap();
            assert_eq!(x.as_str(), "List[x, y, z]");

            let y = x.get_rest().unwrap();
            assert_eq!(y.as_str(), "List[y, z]");

            let z = y.get_rest().unwrap();
            assert_eq!(z.as_str(), "List[z]");

            let a = z.get_rest().unwrap();
            assert_eq!(a.as_str(), "List[]");

            let b = a.get_rest().unwrap();
            assert_eq!(b.as_str(), "List[]");
        }
    }



    mod test_composition {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_composes_LsLe() {
            let list_a = SExpression::new("List") 
                .push_expression(Expression::from("z"))
                .make_generic();

            let list_b = SExpression::new("List") 
                .push_expression(list_a)
                .make_generic();

            assert_eq!(list_b.as_str(), "List[List[z]]")
        }

        #[test]
        fn it_composes_LsLLe() {
            let list_a = SExpression::new("List") 
                .push_expression(Expression::from("z"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(list_a)
                .push_expression(list_b)
                .make_generic();

            assert_eq!(list_c.as_str(), "List[List[z], List[x]]");
        }

        #[test]
        fn it_composes_LsLsLee() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(list_b)
                .make_generic();

            assert_eq!(list_c.as_str(), "List[List[List[x]]]");
        }

        #[test]
        fn it_composes_LpsLpsLpee() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .make_generic();

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]]]");
        }

        #[test]
        fn it_composes_LpsLpsLpepe() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .push_expression(Expression::from("var"))
                .make_generic();

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]], var]")
        }
    }

    mod test_usability {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_composes_clones() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .push_expression(Expression::from("var"))
                .make_generic();

            let list_d = list_c.clone();
            let list_e = SExpression::new("List")
                .push_expression(list_c)
                .push_expression(list_d);

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[x]], var], List[d, List[c, List[x]], var]]")
        }
    }

    mod test_evaluation {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_substitutes_simple() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .push_expression(Expression::from("var"))
                .make_generic();

            let list_d = list_c.clone();
            let list_e = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_c)
                .push_expression(list_d)
                .replace_symbol(Expression::from("d"), Expression::from("2"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[c, List[x]], var], List[2, List[c, List[x]], var]]")
        }

        #[test]
        fn it_substitutes_less_simple() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .push_expression(Expression::from("var"))
                .make_generic();

            let list_d = list_c.clone();
            let list_e = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_c)
                .push_expression(list_d)
                .replace_symbol(Expression::from("d"), Expression::from("2"))
                .replace_symbol(Expression::from("c"), Expression::from("3"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[x]], var], List[2, List[3, List[x]], var]]")
        }

        #[test]
        fn it_substitutes_even_less_simple() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .push_expression(Expression::from("var"))
                .make_generic();

            let list_d = list_c.clone();
            let list_e = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_c)
                .push_expression(list_d)
                .replace_symbol(Expression::from("d"), Expression::from("2"))
                .replace_symbol(Expression::from("c"), Expression::from("3"))
                .replace_symbol(Expression::from("x"), Expression::from("\"Hello\""));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[\"Hello\"]], var], List[2, List[3, List[\"Hello\"]], var]]")
        }

        #[test]
        fn it_substitutes_multichar_symbol() {
            let list_a = SExpression::new("List")
                .push_expression(Expression::from("x"))
                .make_generic();

            let list_b = SExpression::new("List")
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let list_c = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .push_expression(Expression::from("var"))
                .make_generic();

            let list_d = list_c.clone();
            let list_e = SExpression::new("List")
                .push_expression(Expression::from("d"))
                .push_expression(list_c)
                .push_expression(list_d)
                .replace_symbol(Expression::from("d"), Expression::from("2"))
                .replace_symbol(Expression::from("c"), Expression::from("3"))
                .replace_symbol(Expression::from("x"), Expression::from("\"Hello\""))
                .replace_symbol(Expression::from("var"), Expression::from("HelloWorld"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[\"Hello\"]], HelloWorld], List[2, List[3, List[\"Hello\"]], HelloWorld]]")
        }
    }

    mod test_sharing {
        use std::sync::mpsc::channel;
        use std::thread;
        use std::sync::{Arc, Mutex};

        use atom::atom::SimplexAtom;

        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::{BaseExpression, Transmutable};
        
    }
}
