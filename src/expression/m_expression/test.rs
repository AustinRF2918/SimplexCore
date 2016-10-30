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

    mod test_nesting_properties {
        use expression::structure::Expression;
        use expression::s_expression::structure::SExpression;
        use expression::m_expression::structure::MExpression;
        use expression::traits::BaseExpression;

        #[test]
        fn it_allows_nesting() {
            let plus = MExpression::new("Plus")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("b"))
                .toggle_reflexive();

            let pow = MExpression::new("Pow")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("b"))
                .toggle_reflexive();

            let eq = MExpression::new("Equal")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("b"))
                .toggle_reflexive();


            let pythag = MExpression::new("Pythag")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from(MExpression::new("Plus")
                    .toggle_reflexive()
                    .push_meta_variable(Expression::from("a"))
                    .push_meta_variable(Expression::from("b"))
                    .push_expression(Expression::from(pow.evaluate(vec!["a", "2"])))
                    .push_expression(Expression::from(pow.evaluate(vec!["b", "2"]))).evaluate(vec!["a", "b"])))
                .toggle_reflexive();

            assert_eq!(pythag.as_str(), "Pythag[a_, b_] := List[Plus[Pow[a, 2], Pow[b, 2]]]");
        }

        #[test]
        fn it_evals_nesting() {
            let plus = MExpression::new("Plus")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("b"))
                .toggle_reflexive();

            let pow = MExpression::new("Pow")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("b"))
                .toggle_reflexive();

            let eq = MExpression::new("Equal")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from("a"))
                .push_expression(Expression::from("b"))
                .toggle_reflexive();


            let pythag = MExpression::new("Pythag")
                .push_meta_variable(Expression::from("a"))
                .push_meta_variable(Expression::from("b"))
                .push_expression(Expression::from(MExpression::new("Plus")
                    .toggle_reflexive()
                    .push_meta_variable(Expression::from("a"))
                    .push_meta_variable(Expression::from("b"))
                    .push_expression(Expression::from(pow.evaluate(vec!["a", "2"])))
                    .push_expression(Expression::from(pow.evaluate(vec!["b", "2"]))).evaluate(vec!["a", "b"])));

            assert_eq!(pythag.evaluate(vec!["2", "2"]).as_str(), "List[Plus[Pow[2, 2], Pow[2, 2]]]");
        }
    }
}
 
