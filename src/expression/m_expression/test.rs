#[cfg(test)]
mod test {
    mod test_general_functions {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::m_expression::structure::MExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_instantiates() {
            let s_exp = SExpression::new()
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));

            let m_exp = MExpression::new("Plus", vec!["a", "b", "c"], s_exp);
            assert_eq!(m_exp.as_str(), "Plus[a_, b_, c_] := List[a, x, y, z]");
        }

        #[test]
        fn it_evaluates_a() {
            let s_exp = SExpression::new()
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"))
                .push_expression(Expression::from("z"));

            let m_exp = MExpression::new("Plus", vec!["a", "b", "c"], s_exp);
            let new_s_expression = m_exp.evaluate(vec!["1", "2", "3"]);
            assert_eq!(new_s_expression.as_str(), "List[1, x, y, z]");
        }

        #[test]
        fn it_evaluates_b() {
            let s_exp = SExpression::new()
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("x"))
                .push_expression(Expression::from("y"));

            let m_exp = MExpression::new("Plus", vec!["a", "x", "y"], s_exp);
            let new_s_expression = m_exp.evaluate(vec!["1", "2", "3"]);
            assert_eq!(new_s_expression.as_str(), "List[1, 2, 3]");
        }
    }
}
 
