#[cfg(test)]
mod tests {
    mod test_plus_expression {
        use arithmetic::plus::Plus;
        use expression::traits::BaseExpression;
        use expression::traits::SExpression;
        use expression::traits::SExpressionFrom;
        use expression::traits::SExpressionTo;

        use atom::atom::SimplexAtom;

        use atom::numbers::number::Numeric;
        use atom::symbols::symbol::Symbol;

        #[test]
        fn it_instantiates() {
            let my_plus = Plus::new();
            assert_eq!(my_plus.get_expression_type(), "Simplex`MExpression");
            assert_eq!(my_plus.get_head_name(), "Plus");
        }

        #[test]
        fn it_gets_expression_head() {
            let my_plus = Plus::new();
            let x = my_plus.get_head();
            assert_eq!(x.get_expression_type(), "Simplex`Atom");
        }

        #[test]
        fn it_does_a_couple_of_leaf() {
            let mut my_plus = Plus::new();
            my_plus.push_leave(SimplexAtom::from(1));
            my_plus.push_leave(SimplexAtom::from(2));
            my_plus.push_leave(SimplexAtom::from(3));
            my_plus.push_leave(SimplexAtom::from(4));
            let y : Option<SimplexAtom> = my_plus.eval();
            assert_eq!(y.unwrap(), SimplexAtom::from(10));
        }

        #[test]
        fn it_does_mutiple_nodes() {
            let mut a = Plus::new();
            a.push_leave(SimplexAtom::from(1));
            a.push_leave(SimplexAtom::from(2));
            a.push_leave(SimplexAtom::from(3));
            a.push_leave(SimplexAtom::from(4));

            let mut b = Plus::new();
            b.push_leave(SimplexAtom::from(1));
            b.push_leave(SimplexAtom::from(2));
            b.push_leave(SimplexAtom::from(3));
            b.push_leave(SimplexAtom::from(4));

            let mut c = Plus::new();
            c.push_leave(a);
            c.push_leave(b);

            let d : Option<SimplexAtom> = c.eval();
            assert_eq!(d.unwrap(), SimplexAtom::from(20));
        }

        #[test]
        fn it_pushes_a_leave() {
            let mut my_plus = Plus::new();
            let mut my_plus_2 = Plus::new();
            my_plus.push_leave(SimplexAtom::from(1));
            my_plus.push_leave(SimplexAtom::from(2));
            my_plus.push_leave(SimplexAtom::from(3));
            my_plus.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus.push_leave(my_plus_2);
            let y : Option<SimplexAtom> = my_plus.eval();
            assert_eq!(y.unwrap(), SimplexAtom::from(26));
        }
    }

    mod test_minus_expression {
        use arithmetic::subtract::Subtract;
        use arithmetic::plus::Plus;

        use expression::traits::BaseExpression;
        use expression::traits::SExpression;
        use expression::traits::SExpressionFrom;
        use expression::traits::SExpressionTo;

        use atom::atom::SimplexAtom;

        use atom::numbers::number::Numeric;
        use atom::symbols::symbol::Symbol;

        #[test]
        fn it_instantiates() {
            let my_plus = Subtract::new();
            assert_eq!(my_plus.get_expression_type(), "Simplex`MExpression");
            assert_eq!(my_plus.get_head_name(), "Subtract");
        }

        #[test]
        fn it_gets_expression_head() {
            let my_plus = Subtract::new();
            let x = my_plus.get_head();
            assert_eq!(x.get_expression_type(), "Simplex`Atom");
        }

        #[test]
        fn it_pushes_a_leave() {
            let mut my_plus = Subtract::new();
            let mut my_plus_2 = Subtract::new();
            my_plus.push_leave(SimplexAtom::from(1));
            my_plus.push_leave(SimplexAtom::from(2));
            my_plus.push_leave(SimplexAtom::from(3));
            my_plus.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus.push_leave(my_plus_2);
            let y : Option<SimplexAtom> = my_plus.eval();
            assert_eq!(y.unwrap(), SimplexAtom::from(6));
        }
    }

    mod test_both_expressions {
        use arithmetic::subtract::Subtract;
        use arithmetic::plus::Plus;
        use expression::traits::BaseExpression;
        use expression::traits::SExpression;
        use expression::traits::SExpressionFrom;
        use expression::traits::SExpressionTo;

        use atom::atom::SimplexAtom;

        use atom::numbers::number::Numeric;
        use atom::symbols::symbol::Symbol;

        #[test]
        fn it_pushes_a_leave() {
            let mut my_plus = Plus::new();
            let mut my_plus_2 = Subtract::new();
            my_plus.push_leave(SimplexAtom::from(1));
            my_plus.push_leave(SimplexAtom::from(2));
            my_plus.push_leave(SimplexAtom::from(3));
            my_plus.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus_2.push_leave(SimplexAtom::from(4));
            my_plus.push_leave(my_plus_2);
            let y : Option<SimplexAtom> = my_plus.eval();
            assert_eq!(y.unwrap(), SimplexAtom::from(-6));
        }
    }
}
