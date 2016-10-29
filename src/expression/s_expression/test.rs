#[cfg(test)]
mod test {
    mod test_general_functions {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_instantiates() {
            let s_exp = SExpression::new();
        }

        #[test]
        fn it_shows_string() {
            let s_exp = SExpression::new();

            assert_eq!(s_exp.as_str(), "List[]");
        }

        #[test]
        fn it_pushes_expressions() {
            let mut s_exp = SExpression::new()
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));

            assert_eq!(s_exp.as_str(), "List[x, y, z]");
        }

        #[test]
        fn it_pushes_numbers() {
            let mut s_exp = SExpression::new()
                .push_expression(Expression::from("1"))
                .push_expression(Expression::from("2"))
                .push_expression(Expression::from("3"));

            assert_eq!(s_exp.as_str(), "List[1, 2, 3]");
        }
    }

    mod test_composition {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_composes_LsLe() {
            let mut list_a = SExpression::new()
                .push_expression(Expression::from("z"))
                .make_generic();

            let mut list_b = SExpression::new()
                .push_expression(list_a)
                .make_generic();

            assert_eq!(list_b.as_str(), "List[List[z]]")
        }

        #[test]
        fn it_composes_LsLLe() {
            let mut list_a = SExpression::new()
                .push_expression(Expression::from("z"))
                .make_generic();

            let mut list_b = SExpression::new()
                .push_expression(Expression::from("x"))
                .make_generic();

            let mut list_c = SExpression::new()
                .push_expression(list_a)
                .push_expression(list_b)
                .make_generic();

            assert_eq!(list_c.as_str(), "List[List[z], List[x]]");
        }

        #[test]
        fn it_composes_LsLsLee() {
            let mut list_a = SExpression::new()
                .push_expression(Expression::from("x"))
                .make_generic();

            let mut list_b = SExpression::new()
                .push_expression(list_a)
                .make_generic();

            let mut list_c = SExpression::new()
                .push_expression(list_b)
                .make_generic();

            assert_eq!(list_c.as_str(), "List[List[List[x]]]");
        }

        #[test]
        fn it_composes_LpsLpsLpee() {
            let mut list_a = SExpression::new()
                .push_expression(Expression::from("x"))
                .make_generic();

            let mut list_b = SExpression::new()
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let mut list_c = SExpression::new()
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .make_generic();

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]]]");
        }

        #[test]
        fn it_composes_LpsLpsLpepe() {
            let mut list_a = SExpression::new()
                .push_expression(Expression::from("x"))
                .make_generic();

            let mut list_b = SExpression::new()
                .push_expression(Expression::from("c"))
                .push_expression(list_a)
                .make_generic();

            let mut list_c = SExpression::new()
                .push_expression(Expression::from("d"))
                .push_expression(list_b)
                .push_expression(Expression::from("var"))
                .make_generic();

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]], var]")
        }
    }
}
