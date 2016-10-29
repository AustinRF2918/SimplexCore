use std::str::FromStr;
use std::borrow::Cow;

use atom::atom::SimplexAtom;

use expression::traits::BaseExpression;
use expression::s_expression::structure::SExpression;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression {
    List(SExpression),
    Atomic(SimplexAtom)
}

impl Expression {
    pub fn new_primitive(s: &str) -> Expression {
        Expression::Atomic(SimplexAtom::from(s))
    }

    pub fn new_list(l: SExpression) -> Expression {
        Expression::List(l)
    }

    pub fn to_string(&self) -> String {
        match self {
            &Expression::List(ref l) => l.to_string().clone(),
            &Expression::Atomic(ref a) => a.to_string().clone(),
        }
    }

    pub fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }
}

impl<'a> From<&'a str> for Expression {
    fn from(s: &str) -> Expression {
        Expression::Atomic(SimplexAtom::from(s))
    }
}

impl From<SimplexAtom> for Expression {
    fn from(a: SimplexAtom) -> Expression {
        Expression::Atomic(a)
    }
}

impl From<SExpression> for Expression {
    fn from(s: SExpression) -> Expression {
        Expression::List(s)
    }
}

