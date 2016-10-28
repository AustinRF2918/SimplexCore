use atom::atom::SimplexAtom;

use expression::traits::BaseExpression;
use expression::s_expression::structure::SExpression;

use arithmetic::plus::Plus;
use arithmetic::subtract::Subtract;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression {
    Add(Plus),
    Sub(Subtract),
    List(SExpression),
    Atomic(SimplexAtom)
}

impl Expression {
    pub fn new_primitive(s: &str) -> Expression {
        Expression::Atomic(SimplexAtom::from(s))
    }

    pub fn new_builtin(s: &str) -> Expression {
        if s == "Plus" {
            Expression::Add(Plus::new())
        } else {
            Expression::Sub(Subtract::new())
        }
    }

    pub fn new_list(l: SExpression) -> Expression {
        Expression::List(l)
    }

    pub fn to_string(&self) -> String {
        match self {
            &Expression::Add(_) => "Add".to_string(),
            &Expression::Sub(_) => "Sub".to_string(),
            &Expression::List(ref l) => l.to_string().clone(),
            &Expression::Atomic(ref a) => a.to_string().clone(),
        }
    }
}

/*
pub fn eval(e: Expression) -> Expression {
    
}
*/
