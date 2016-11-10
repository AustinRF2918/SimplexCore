#[cfg(test)]
mod test {
    mod test_general_functions {
        use expression::list::structure::SimplexList;
        use expression::atom::structure::SimplexAtom;
        use expression::traits::BaseExpression;
        use expression::structure::SimplexPointer;

        #[test]
        fn it_instantiates() {
            {
            let s_exp = SimplexList::new("List");
            assert_eq!(s_exp.as_str(), "List[]");
            }
        }

        #[test]
        fn it_shows_string() {
            let s_exp = SimplexList::new("List");
            assert_eq!(s_exp.as_str(), "List[]");
        }

        #[test]
        fn it_pushes_lists() {
            let s_exp = SimplexList::new("List") 
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            assert_eq!(s_exp.as_str(), "List[x, y, z]");
        }

        #[test]
        fn it_pushes_numbers() {
            let s_exp = SimplexList::new("List") 
                .push(&SimplexPointer::from("1"))
                .push(&SimplexPointer::from("2"))
                .push(&SimplexPointer::from("3"));

            assert_eq!(s_exp.as_str(), "List[1, 2, 3]");
        }

        #[test]
        fn it_shows_changes() {
            let mut x = SimplexPointer::from("2");

            let list_a = SimplexList::new("List") 
                .push(&x)
                .push(&SimplexPointer::from("2"))
                .push(&SimplexPointer::from("3"));

            x.replace_symbol(&SimplexAtom::from("2"), &SimplexAtom::from("1"));

            let list_b = SimplexList::new("List") 
                .push(&x)
                .push(&SimplexPointer::from("2"))
                .push(&SimplexPointer::from("3"));


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
        use expression::structure::SimplexPointer;

        #[test]
        fn it_gets_rest() {
            let m_exp = SimplexList::new("List")
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            let y = m_exp.get_rest().unwrap();
            assert_eq!(m_exp.get_rest().unwrap().as_str(), "List[x, y, z]");
            assert_eq!(y.get_rest().unwrap().as_str(), "List[y, z]");
        }

        #[test]
        fn it_gets_rest_recursively_one_more() {
            let m_exp = SimplexList::new("List")
                .push(&SimplexPointer::from("b"))
                .push(&SimplexPointer::from("a"))
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            let x  = m_exp.get_rest().unwrap();
            assert_eq!(x.as_str(), "List[a, x, y, z]");

            let y = x.get_rest().unwrap();
            assert_eq!(y.as_str(), "List[x, y, z]");

            let z = y.get_rest().unwrap();
            assert_eq!(z.as_str(), "List[y, z]");

            let a = z.get_rest().unwrap();
            assert_eq!(a.as_str(), "List[z]");

            let b = a.get_rest();
            assert_eq!(b, None);
        }

        #[test]
        fn it_gets_rest_recursively_normal() {

            {
                let mut m_exp = SimplexList::new("List")
                    .push(&SimplexPointer::from("a"))
                    .push(&SimplexPointer::from("x"))
                    .push(&SimplexPointer::from("y"))
                    .push(&SimplexPointer::from("z"));

                let mut x  = m_exp.get_rest().unwrap();
                assert_eq!(x.as_str(), "List[x, y, z]");

                let mut y = x.get_rest().unwrap();
                assert_eq!(y.as_str(), "List[y, z]");

                let mut z = y.get_rest().unwrap();
                assert_eq!(z.as_str(), "List[z]");

                let mut a = z.get_rest();
                assert_eq!(a, None);
            }

        }

        #[test]
        fn it_gets_rest_recursively_one_less() {
            let m_exp = SimplexList::new("List")
                .push(&SimplexPointer::from("x"))
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            let x  = m_exp.get_rest().unwrap();
            assert_eq!(x.as_str(), "List[y, z]");

            let y = x.get_rest().unwrap();
            assert_eq!(y.as_str(), "List[z]");

            let z = y.get_rest();
            assert_eq!(z, None);
        }

        #[test]
        fn it_gets_rest_recursively_two_less() {
            let mut m_exp = SimplexList::new("List")
                .push(&SimplexPointer::from("y"))
                .push(&SimplexPointer::from("z"));

            let mut x  = m_exp.get_rest().unwrap();
            assert_eq!(x.as_str(), "List[z]");

            let y = x.get_rest();
            assert_eq!(y, None);

            println!("Destructing All.");
        }

        #[test]
        fn it_gets_rest_recursively_three_less() {
            let m_exp = SimplexList::new("List")
                .push(&SimplexPointer::from("y"));

            let x  = m_exp.get_rest();
            assert_eq!(x, None);
        }
    }

    mod test_composition {
        use expression::list::structure::SimplexList;
        use expression::traits::BaseExpression;
        use expression::structure::SimplexPointer;

        #[allow(non_snake_case)]
        #[test]
        fn it_composes_LsLe() {
            let list_a = SimplexList::new("List") 
                .push(&SimplexPointer::from("z"));

            let list_b = SimplexList::new("List") 
                .push(&SimplexPointer::from(list_a));

            assert_eq!(list_b.as_str(), "List[List[z]]")
        }

        #[allow(non_snake_case)]
        #[test]
        fn it_composes_LsLLe() {
            let list_a = SimplexList::new("List") 
                .push(&SimplexPointer::from("z"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from(list_a))
                .push(&SimplexPointer::from(list_b));

            assert_eq!(list_c.as_str(), "List[List[z], List[x]]");
        }

        #[allow(non_snake_case)]
        #[test]
        fn it_composes_LsLsLee() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from(list_b));

            assert_eq!(list_c.as_str(), "List[List[List[x]]]");
        }

        #[allow(non_snake_case)]
        #[test]
        fn it_composes_LpsLpsLpee() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b));

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]]]");
        }

        #[allow(non_snake_case)]
        #[test]
        fn it_composes_LpsLpsLpepe() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b))
                .push(&SimplexPointer::from("var"));

            assert_eq!(list_c.as_str(), "List[d, List[c, List[x]], var]")
        }
    }

    mod test_usability {
        use expression::list::structure::SimplexList;
        use expression::traits::BaseExpression;
        use expression::atom::structure::SimplexAtom;
        use expression::structure::SimplexPointer;

        #[test]
        fn it_composes_clones_a() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b))
                .push(&SimplexPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&SimplexPointer::from(list_c))
                .push(&SimplexPointer::from(list_d));

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[x]], var], List[d, List[c, List[x]], var]]")
        }

        #[test]
        fn it_composes_clones_with_replacement() {
            let mut x = SimplexPointer::from("x");

            let list_a = SimplexList::new("List")
                .push(&x);

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexPointer::from(SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b))
                .push(&SimplexPointer::from("var")));

            let list_e = SimplexList::new("List")
                .push(&list_c)
                .push(&list_c);

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[x]], var], List[d, List[c, List[x]], var]]");

            x.replace_symbol(&SimplexAtom::from("x"), &SimplexAtom::from("y"));

            assert_eq!(list_e.as_str(), "List[List[d, List[c, List[y]], var], List[d, List[c, List[y]], var]]");
        }
    }

    mod test_evaluation {
        use expression::list::structure::SimplexList;
        use expression::traits::BaseExpression;
        use expression::structure::SimplexPointer;

        #[test]
        fn it_substitutes_simple() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexPointer::from(SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b))
                .push(&SimplexPointer::from("var")));

            let list_e = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&list_c)
                .push(&list_c)
                .replace_symbol(&SimplexPointer::from("d"), &SimplexPointer::from("2"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[c, List[x]], var], List[2, List[c, List[x]], var]]")
        }

        #[test]
        fn it_substitutes_less_simple() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b))
                .push(&SimplexPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_c))
                .push(&SimplexPointer::from(list_d))
                .replace_symbol(&SimplexPointer::from("d"), &SimplexPointer::from("2"))
                .replace_symbol(&SimplexPointer::from("c"), &SimplexPointer::from("3"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[x]], var], List[2, List[3, List[x]], var]]")
        }

        #[test]
        fn it_substitutes_even_less_simple() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b))
                .push(&SimplexPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_c))
                .push(&SimplexPointer::from(list_d))
                .replace_symbol(&SimplexPointer::from("d"), &SimplexPointer::from("2"))
                .replace_symbol(&SimplexPointer::from("c"), &SimplexPointer::from("3"))
                .replace_symbol(&SimplexPointer::from("x"), &SimplexPointer::from("\"Hello\""));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[\"Hello\"]], var], List[2, List[3, List[\"Hello\"]], var]]")
        }

        #[test]
        fn it_substitutes_multichar_symbol() {
            let list_a = SimplexList::new("List")
                .push(&SimplexPointer::from("x"));

            let list_b = SimplexList::new("List")
                .push(&SimplexPointer::from("c"))
                .push(&SimplexPointer::from(list_a));

            let list_c = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_b))
                .push(&SimplexPointer::from("var"));

            let list_d = list_c.clone();
            let list_e = SimplexList::new("List")
                .push(&SimplexPointer::from("d"))
                .push(&SimplexPointer::from(list_c))
                .push(&SimplexPointer::from(list_d))
                .replace_symbol(&SimplexPointer::from("d"), &SimplexPointer::from("2"))
                .replace_symbol(&SimplexPointer::from("c"), &SimplexPointer::from("3"))
                .replace_symbol(&SimplexPointer::from("x"), &SimplexPointer::from("\"Hello\""))
                .replace_symbol(&SimplexPointer::from("var"), &SimplexPointer::from("HelloWorld"));

            assert_eq!(list_e.as_str(), "List[2, List[2, List[3, List[\"Hello\"]], HelloWorld], List[2, List[3, List[\"Hello\"]], HelloWorld]]")
        }
    }
}
