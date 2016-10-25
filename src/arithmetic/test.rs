#[cfg(test)]
mod tests {
    mod test_plus_expression {
        use arithmetic::plus::Plus;
        use expression::traits::BaseExpression;
        use expression::traits::SExpression;
        use expression::traits::SExpressionFrom;
        use expression::traits::BuiltinExpression;

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
            let x = my_plus.eval();
            println!("{}", x.to_string());
            let y = x.reduce_expression();
            assert_eq!(y.unwrap(), SimplexAtom::from(16));
        }
    }

    mod test_minus_expression {
        use arithmetic::subtract::Subtract;
        use arithmetic::plus::Plus;

        use expression::traits::BaseExpression;
        use expression::traits::SExpression;
        use expression::traits::SExpressionFrom;
        use expression::traits::BuiltinExpression;

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
            let x = my_plus.eval();
            println!("{}", x.to_string());
            let y = x.reduce_expression();
            assert_eq!(y.unwrap(), SimplexAtom::from(-16));
        }
    }

    mod test_both_expressions {
        use arithmetic::subtract::Subtract;
        use arithmetic::plus::Plus;
        use expression::traits::BaseExpression;
        use expression::traits::SExpression;
        use expression::traits::SExpressionFrom;
        use expression::traits::BuiltinExpression;

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
            let x = my_plus.eval();
            my_plus.push_leave(x);
            let y = my_plus.reduce_expression();
            assert_eq!(y.unwrap(), SimplexAtom::from(-13));
        }
    }
}