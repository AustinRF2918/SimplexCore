#[cfg(test)]
mod tests {
    mod test_base_expression {
        use std::sync::{Arc, Mutex};
        use std::rc::Rc;
        use expression::structures::structure::BaseExpression;

        #[test]
        fn it_instantiates_symbol() {
            let plus = CoreExpression::new("Add");
            assert_eq!(plus.get_head_name(), "Simplex`Add");
        }

        #[test]
        fn it_instantiates_num() {
            let num = CoreExpression::new("22");
            assert_eq!(num.get_head_name(), "Simplex`Integer");
        }

        #[test]
        fn it_instantiates_string() {
            let st = CoreExpression::new("\"Hello\"");
            assert_eq!(st.get_head_name(), "Simplex`String");
        }

        #[test]
        fn pushes_leaves() {
            let mut sa = Rc::new(CoreExpression::new("Plus"));

            let a = Rc::new(CoreExpression::new("1"));
            let b = Rc::new(CoreExpression::new("2"));
            let c = Rc::new(CoreExpression::new("3"));

            Rc::get_mut(&mut sa).unwrap().push_leave(a);
            Rc::get_mut(&mut sa).unwrap().push_leave(b);
            Rc::get_mut(&mut sa).unwrap().push_leave(c);

            let m = CoreExpression::new("6");

            assert_eq!(sa.to_string(), "Plus[1, 2, 3]");
            assert_eq!(sa.eval().to_string(), CoreExpression::new("6").to_string());
        }
        // TODO: Must update API call once Rule is implemented.
        #[test]
        fn it_iterates_rules() {
            // let x = CoreExpression::new();
            // assert_eq!(x.iterate_rules(), None);
            //
        }

        // Test Cluster: As the Rule API has not been
        // developed as of yet, we must stub this as
        // a minimalized API that we do not need to
        // pass rules over to: As a result, I only
        // test cases that given any rule would produce
        // the same output.
        //

        // Must update API call once Rule is implemented.
        //
        // #[test]
        // fn it_applies_rules_case_1() {
        // let x = CoreExpression::new();
        // assert_eq!(x.apply_rules(1, (Some(2), None)), None);
        // }
        //
        // fn it_applies_rules_case_3() {
        // let x = CoreExpression::new();
        // assert_eq!(x.apply_rules(2, (None, Some(1))), None);
        // }
        //

        // Case 4, 5 is guarenteed to function if
        // cases 1 and 3 functions.
    }
}
