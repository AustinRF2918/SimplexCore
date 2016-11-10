#[cfg(test)]
mod test {
    mod test_intrinsics {
        use expression::structure::SimplexPointer;
        use expression::list::structure::SimplexList;
        use expression::function::structure::SimplexFunction;
        use expression::atom::structure::SimplexAtom;
        use expression::traits::BaseExpression;

        #[test]
        fn it_gets_rest() {
            let m_exp = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push_meta_variable(SimplexAtom::from("c"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            assert_eq!(m_exp.get_rest().unwrap().as_str(), "Plus[b_, c_] := List[a, x, y, z]");
        }

        #[test]
        fn it_gets_rest_recursively_once() {
            let m_exp = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push_meta_variable(SimplexAtom::from("c"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            let a  = m_exp.get_rest().unwrap();
            assert_eq!(a.as_str(), "Plus[b_, c_] := List[a, x, y, z]");

            let b = a.get_rest().unwrap();
            assert_eq!(b.as_str(), "Plus[c_] := List[a, x, y, z]");

            let c = b.get_rest().unwrap();
            assert_eq!(c.as_str(), "Plus[] := List[a, x, y, z]");

            let d = c.get_rest().unwrap();
            assert_eq!(d.as_str(), "Plus[] := List[x, y, z]");

            let e = d.get_rest().unwrap();
            assert_eq!(e.as_str(), "Plus[] := List[y, z]");

            let f = e.get_rest().unwrap();
            assert_eq!(f.as_str(), "Plus[] := List[z]");

            let g = f.get_rest().unwrap();
            assert_eq!(g.as_str(), "Plus");

            let h = g.get_rest();
            assert_eq!(h, None);
        }
    }

    mod test_general_functions {
        use expression::structure::SimplexPointer;
        use expression::list::structure::SimplexList;
        use expression::function::structure::SimplexFunction;
        use expression::traits::BaseExpression;
        use expression::atom::structure::SimplexAtom;

        #[test]
        fn it_instantiates() {
            let m_exp = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push_meta_variable(SimplexAtom::from("c"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            assert_eq!(m_exp.as_str(), "Plus[a_, b_, c_] := List[a, x, y, z]");
        }

        #[test]
        fn it_evaluates_a() {
            let m_exp = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push_meta_variable(SimplexAtom::from("c"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            let new_list = m_exp.evaluate(&vec![SimplexPointer::from("1"), SimplexPointer::from("2"), SimplexPointer::from("3")]);
            assert_eq!(new_list.as_str(), "List[1, x, y, z]");
        }

        #[test]
        fn it_evaluates_b() {
            let m_exp = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push_meta_variable(SimplexAtom::from("c"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"))
                .push(&SimplexPointer::from("c"));

            let new_list = m_exp.evaluate(&vec![SimplexPointer::from("1"), SimplexPointer::from("2"), SimplexPointer::from("3")]);
            assert_eq!(new_list.as_str(), "List[1, 2, 3]");
        }
    }

    mod test_nesting_properties {
        use expression::structure::SimplexPointer;
        use expression::list::structure::SimplexList;
        use expression::function::structure::SimplexFunction;
        use expression::traits::BaseExpression;
        use expression::atom::structure::SimplexAtom;

        #[test]
        fn it_allows_simple_nesting() {
            let mut plus = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"));

            plus.toggle_base_evaluation();

            let simple = SimplexFunction::new("Simple")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from(plus.evaluate(&vec![])));
                    

            assert_eq!(simple.as_str(), "Simple[a_, b_] := List[Plus[a, b]]");
        }

        #[test]
        fn it_allows_nesting() {
            let mut plus = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"));

            let mut pow = SimplexFunction::new("Pow")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"));

            let mut eq = SimplexFunction::new("Equal")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"));

            plus.toggle_base_evaluation();
            pow.toggle_base_evaluation();
            eq.toggle_base_evaluation();

            let pythag = SimplexFunction::new("Pythag")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from(
                    plus.evaluate(
                        &vec![
                            pow.evaluate(
                                &vec![
                                    SimplexPointer::from("a"),
                                    SimplexPointer::from("2"),
                                ]
                            ),

                            pow.evaluate(
                                &vec![
                                    SimplexPointer::from("b"),
                                    SimplexPointer::from("2"),
                                ]
                            )
                            ]
                        )
                    )
                );

            assert_eq!(pythag.as_str(), "Pythag[a_, b_] := List[Plus[Pow[a, 2], Pow[b, 2]]]");
        }

        #[test]
        fn it_evals_nesting() {
            let mut plus = SimplexFunction::new("Plus")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"));

            let mut pow = SimplexFunction::new("Pow")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"));

            let mut eq = SimplexFunction::new("Equal")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("b"));

            plus.toggle_base_evaluation();
            pow.toggle_base_evaluation();
            eq.toggle_base_evaluation();

            let pythag = SimplexFunction::new("Pythag")
                .push_meta_variable(SimplexAtom::from("a"))
                .push_meta_variable(SimplexAtom::from("b"))
                .push(&SimplexPointer::from(plus.clone().evaluate(
                    &vec![
                        SimplexPointer::from(pow.clone().evaluate(
                            &vec![SimplexPointer::from("a"), SimplexPointer::from("2")],
                        )),
                        SimplexPointer::from(pow.clone().evaluate(
                            &vec![SimplexPointer::from("b"), SimplexPointer::from("2")],
                        ))
                    ], 
                )));
             

            assert_eq!(pythag.evaluate(&vec![SimplexPointer::from("2"), SimplexPointer::from("2")]).as_str(), "List[Plus[Pow[2, 2], Pow[2, 2]]]");
        }
    }
}
 
