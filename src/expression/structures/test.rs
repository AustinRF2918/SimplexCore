#[cfg(test)]
mod tests {
    mod test_base_expression {
        use expression::structures::structure::BaseExpression;

        #[test]
        fn it_instantiates() {
            let x = BaseExpression::new();
            assert_eq!(x.is_formatted, true);
            assert_eq!(x.pattern_sequence, false);
        }

        // TODO: Must update API call once Rule is implemented.
        #[test]
        fn it_iterates_rules() {
            let x = BaseExpression::new();
            assert_eq!(x.iterate_rules(), None);
        }

        /* 
            Test Cluster: As the Rule API has not been
            developed as of yet, we must stub this as
            a minimalized API that we do not need to 
            pass rules over to: As a result, I only 
            test cases that given any rule would produce
            the same output.
        */

        // Must update API call once Rule is implemented.
        #[test]
        fn it_applies_rules_case_1() {
            let x = BaseExpression::new();
            assert_eq!(x.apply_rules(1, (Some(2), None)), None);
        }

        fn it_applies_rules_case_3() {
            let x = BaseExpression::new();
            assert_eq!(x.apply_rules(2, (None, Some(1))), None);
        }

        // Case 4, 5 is guarenteed to function if
        // cases 1 and 3 functions.
    }
}
