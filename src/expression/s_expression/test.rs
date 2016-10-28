#[cfg(test)]
mod test {
    mod test_general_functions {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_instantiates() {
            let x = SExpression::new();
        }

        #[test]
        fn it_shows_string() {
            let x = SExpression::new();
            assert_eq!(x.to_string().as_str(), "List[]");
        }

        #[test]
        fn it_pushes_expressions() {
            let mut x = SExpression::new();
            x.push_expression(Expression::new_primitive("x"));
            x.push_expression(Expression::new_primitive("y"));
            x.push_expression(Expression::new_primitive("z"));
            assert_eq!(x.to_string().as_str(), "List[x, y, z]");
        }

        #[test]
        fn it_pushes_numbers() {
            let mut x = SExpression::new();
            x.push_expression(Expression::new_primitive("1"));
            x.push_expression(Expression::new_primitive("2"));
            x.push_expression(Expression::new_primitive("3"));
            assert_eq!(x.to_string().as_str(), "List[1, 2, 3]");
        }
    }

    mod test_composition {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_composes_LsLe() {
            let mut s_a = SExpression::new();
            let mut s_b = SExpression::new();

            s_a.push_expression(Expression::new_primitive("z"));

            let mut list_a = Expression::new_list(s_a);

            s_b.push_expression(list_a);
            assert_eq!(s_b.to_string().as_str(), "List[List[z]]")
        }

        #[test]
        fn it_composes_LsLLe() {
            let mut s_a = SExpression::new();
            let mut s_b = SExpression::new();
            let mut s_c = SExpression::new();

            s_a.push_expression(Expression::new_primitive("z"));
            s_c.push_expression(Expression::new_primitive("x"));

            let mut list_a = Expression::new_list(s_a);
            let mut list_c = Expression::new_list(s_c);

            s_b.push_expression(list_a);
            s_b.push_expression(list_c);

            assert_eq!(s_b.to_string().as_str(), "List[List[z], List[x]]")
        }

        #[test]
        fn it_composes_LsLsLee() {
            let mut s_a = SExpression::new();
            let mut s_b = SExpression::new();
            let mut s_c = SExpression::new();

            s_c.push_expression(Expression::new_primitive("x"));

            let mut list_c = Expression::new_list(s_c);

            s_a.push_expression(list_c);
            let mut list_a = Expression::new_list(s_a);

            s_b.push_expression(list_a);

            assert_eq!(s_b.to_string().as_str(), "List[List[List[x]]]")
        }

        #[test]
        fn it_composes_LpsLpsLpee() {
            let mut s_a = SExpression::new();
            let mut s_b = SExpression::new();
            let mut s_c = SExpression::new();

            s_b.push_expression(Expression::new_primitive("d"));
            s_a.push_expression(Expression::new_primitive("c"));
            s_c.push_expression(Expression::new_primitive("x"));

            let mut list_c = Expression::new_list(s_c);

            s_a.push_expression(list_c);
            let mut list_a = Expression::new_list(s_a);

            s_b.push_expression(list_a);

            assert_eq!(s_b.to_string().as_str(), "List[d, List[c, List[x]]]")
        }

        #[test]
        fn it_composes_LpsLpsLpepe() {
            let mut s_b = SExpression::new();
            s_b.push_expression(Expression::new_primitive("d"));

            let mut s_a = SExpression::new();
            s_a.push_expression(Expression::new_primitive("c"));

            let mut s_c = SExpression::new();
            s_c.push_expression(Expression::new_primitive("x"));
            let list_c = Expression::new_list(s_c);

            s_a.push_expression(list_c);

            let mut list_a = Expression::new_list(s_a);
            s_b.push_expression(list_a);

            s_b.push_expression(Expression::new_primitive("var"));

            assert_eq!(s_b.to_string().as_str(), "List[d, List[c, List[x]], var]")
        }
    }
}
