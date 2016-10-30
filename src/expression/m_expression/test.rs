#[cfg(test)]
mod test {
    mod test_general_functions {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::m_expression::structure::MExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_instantiates() {
            let m_exp = MExpression::new("Plus")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_meta_variable(Expression::from("c"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));

            assert_eq!(m_exp.as_str(), "Plus[a_, b_, c_] := List[a, x, y, z]");
        }

        #[test]
        fn it_evaluates_a() {
            let m_exp = MExpression::new("Plus")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_meta_variable(Expression::from("c"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));


            let new_s_expression = m_exp.evaluate(vec!["1", "2", "3"]);
            assert_eq!(new_s_expression.as_str(), "List[1, x, y, z]");
        }

        #[test]
        fn it_evaluates_b() {
            let m_exp = MExpression::new("Plus")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_meta_variable(Expression::from("c"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("b"))
                .push_expression(Expression::from("c"));

            let new_s_expression = m_exp.evaluate(vec!["1", "2", "3"]);
            assert_eq!(new_s_expression.as_str(), "List[1, 2, 3]");
        }
    }
}
 
