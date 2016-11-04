#[cfg(test)]
mod test {
    mod test_general_functions {
        use expression::list::structure::SimplexList;
        use atom::atom::SimplexAtom;
        use expression::traits::BaseExpression;
        use expression::structure::ExpressionPointer;

        #[test]
        fn it_instantiates() {
            let s_exp = SimplexList::new("List");
        }

        #[test]
        fn it_shows_string() {
            let s_exp = SimplexList::new("List");
            assert_eq!(s_exp.as_str(), "List[]");
        }

        #[test]
        fn it_pushelists() {
            let s_exp = SimplexList::new("List") 
                .push(&ExpressionPointer::from("x"))
                .push(&ExpressionPointer::from("y"))
                .push(&ExpressionPointer::from("z"));

            assert_eq!(s_exp.as_str(), "List[x, y, z]");
        }

        #[test]
        fn it_pushes_numbers() {
            let s_exp = SimplexList::new("List") 
                .push(&ExpressionPointer::from("1"))
                .push(&ExpressionPointer::from("2"))
                .push(&ExpressionPointer::from("3"));

            assert_eq!(s_exp.as_str(), "List[1, 2, 3]");
        }

        #[test]
        fn it_shows_changes() {
            let mut x = ExpressionPointer::from("2");

            let list_a = SimplexList::new("List") 
                .push(&x)
                .push(&ExpressionPointer::from("2"))
                .push(&ExpressionPointer::from("3"));

            x.replace_symbol(&SimplexAtom::from("2"), &SimplexAtom::from("1"));

            let list_b = SimplexList::new("List") 
                .push(&x)
                .push(&ExpressionPointer::from("2"))
                .push(&ExpressionPointer::from("3"));


            assert_eq!(list_a.as_str(), "List[1, 2, 3]");
            assert_eq!(list_b.as_str(), "List[1, 2, 3]");

            x.replace_symbol(&SimplexAtom::from("1"), &SimplexAtom::from("9"));

            assert_eq!(list_a.as_str(), "List[9, 2, 3]");
            assert_eq!(list_b.as_str(), "List[9, 2, 3]");
        }
    }

mod test_intrinsics {
        use expression::list::structure::SimplexList;
        use expression::traits::BaseExpression;
        use atom::atom::SimplexAtom;
        use expression::structure::ExpressionPointer;

        #[test]
        fn it_gets_rest() {
            let m_exp = SimplexList::new("List")
                .push(&ExpressionPointer::from("a"))
                .push(&ExpressionPointer::from("x"))
                .push(&ExpressionPointer::from("y"))
                .push(&ExpressionPointer::from("z"));
            assert_eq!(m_exp.get_rest().unwrap().as_str(), "List[x, y, z]");
        }

        #[test]
        fn it_gets_rest_recursively_once() {
            let m_exp = SimplexList::new("List")
                .push(&ExpressionPointer::from("a"))
                .push(&ExpressionPointer::from("x"))
                .push(&ExpressionPointer::from("y"))
                .push(&ExpressionPointer::from("z"));

            let x  = m_exp.get_rest().unwrap();
            assert_eq!(x.as_str(), "List[x, y, z]");

            let y = x.get_rest().unwrap();
            assert_eq!(y.as_str(), "List[y, z]");

            let z = y.get_rest().unwrap();
            assert_eq!(z.as_str(), "List[z]");

            let a = z.get_rest();
            assert_eq!(a, None);
        }
    }

    mod test_composition {
        use expression::list::structure::SimplexList;
        use expression::traits::BaseExpression;
        use atom::atom::SimplexAtom;
        use expression::structure::ExpressionPointer;

        #[test]
        fn it_composes_LsLe() {
            let list_a = SimplexList::new("List") 
                .push(&ExpressionPointer::from("z"));

            let list_b = SimplexList::new("List") 
                .push(&ExpressionPointer::from(list_a));

            assert_eq!(list_b.as_str(), "List[List[z]]")
        }

        #[test]
        fn it_composes_LsLLe() {
            let list_a = SimplexList::new("List") 
                .push(&ExpressionPointer::from("z"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from(list_a))
                .push(&ExpressionPointer::from(list_b));

            assert_eq!(list_c.as_str(), "List[List[z], List[x]]");
        }

        #[test]
        fn it_composes_LsLsLee() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from(list_b));

            assert_eq!(list_c.as_str(), "List[List[List[x]]]");
        }

        #[test]
        fn it_composes_LpsLpsLpee() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b));

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]]]");
        }

        #[test]
        fn it_composes_LpsLpsLpepe() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var"));

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]], var]")
        }
    }

    mod test_usability {
        use expression::list::structure::SimplexList;
        use expression::traits::BaseExpression;
        use atom::atom::SimplexAtom;
        use expression::structure::ExpressionPointer;

        #[test]
        fn it_composes_clones() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&ExpressionPointer::from(list_c))
                .push(&ExpressionPointer::from(list_d));

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[x]], var], List[d, List[c, List[x]], var]]")
        }

        #[test]
        fn it_composes_clones_with_replacement() {
            let mut x = ExpressionPointer::from("x");

            let list_a = SimplexList::new("List")
                .push(&x);

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = ExpressionPointer::from(SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var")));

            let list_e = SimplexList::new("List")
                .push(&list_c)
                .push(&list_c);

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[x]], var], List[d, List[c, List[x]], var]]");

            x.replace_symbol(&SimplexAtom::from("x"), &SimplexAtom::from("y"));

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[y]], var], List[d, List[c, List[y]], var]]");
        }

        #[test]
        fn it_composes_clones_with_complex_replacement() {
            let mut x = ExpressionPointer::from("x");

            let list_a = ExpressionPointer::from(SimplexList::new("List")
                .push(&x));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&list_a);

            let list_c = ExpressionPointer::from(SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var")));

            let list_e = SimplexList::new("List")
                .push(&list_c)
                .push(&list_c);

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[x]], var], List[d, List[c, List[x]], var]]");

            x.replace_symbol(&SimplexAtom::from("x"), &ExpressionPointer::from(SimplexList::new("List")));

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[List[]]], var], List[d, List[c, List[List[]]], var]]");
        }
    }

    mod test_evaluation {
        use expression::list::structure::SimplexList;
        use expression::traits::BaseExpression;
        use atom::atom::SimplexAtom;
        use expression::structure::ExpressionPointer;

        #[test]
        fn it_substitutes_simple() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = ExpressionPointer::from(SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var")));

            let list_e = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&list_c)
                .push(&list_c)
                .replace_symbol(&ExpressionPointer::from("d"), &ExpressionPointer::from("2"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[c, List[x]], var], List[2, List[c, List[x]], var]]")
        }

        #[test]
        fn it_substitutes_less_simple() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_c))
                .push(&ExpressionPointer::from(list_d))
                .replace_symbol(&ExpressionPointer::from("d"), &ExpressionPointer::from("2"))
                .replace_symbol(&ExpressionPointer::from("c"), &ExpressionPointer::from("3"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[x]], var], List[2, List[3, List[x]], var]]")
        }

        #[test]
        fn it_substitutes_even_less_simple() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_c))
                .push(&ExpressionPointer::from(list_d))
                .replace_symbol(&ExpressionPointer::from("d"), &ExpressionPointer::from("2"))
                .replace_symbol(&ExpressionPointer::from("c"), &ExpressionPointer::from("3"))
                .replace_symbol(&ExpressionPointer::from("x"), &ExpressionPointer::from("\"Hello\""));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[\"Hello\"]], var], List[2, List[3, List[\"Hello\"]], var]]")
        }

        #[test]
        fn it_substitutes_multichar_symbol() {
            let list_a = SimplexList::new("List")
                .push(&ExpressionPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&ExpressionPointer::from("c"))
                .push(&ExpressionPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_b))
                .push(&ExpressionPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&ExpressionPointer::from("d"))
                .push(&ExpressionPointer::from(list_c))
                .push(&ExpressionPointer::from(list_d))
                .replace_symbol(&ExpressionPointer::from("d"), &ExpressionPointer::from("2"))
                .replace_symbol(&ExpressionPointer::from("c"), &ExpressionPointer::from("3"))
                .replace_symbol(&ExpressionPointer::from("x"), &ExpressionPointer::from("\"Hello\""))
                .replace_symbol(&ExpressionPointer::from("var"), &ExpressionPointer::from("HelloWorld"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[\"Hello\"]], HelloWorld], List[2, List[3, List[\"Hello\"]], HelloWorld]]")
        }
    }
}
