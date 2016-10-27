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
}
