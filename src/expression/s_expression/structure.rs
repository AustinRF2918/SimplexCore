use std::borrow::Cow;

use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::Expression;

use atom::atom::SimplexAtom;


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SExpression {
    expressions: LinkedList<Expression>,
}

impl SExpression {
    pub fn new() -> SExpression {
        let mut expression_list : LinkedList<Expression> = LinkedList::new();
        expression_list.push_back(Expression::new_primitive("List"));

        SExpression {
            expressions: expression_list,
        }
    }

    pub fn push_expression(mut self, e: Expression) -> SExpression {
        self.expressions.push_back(e);
        self
    }

    pub fn replace_symbol(mut self, symbol: &Expression, new: &Expression) -> SExpression {
        for i in &mut self.expressions {
            match i {
                &mut Expression::Atomic(_) => {
                    if i.as_str() == symbol.as_str() {
                        *i = new.clone();
                    }
                }

                &mut Expression::List(ref mut l) => {
                    let mut r = l.clone();
                    *l = r.replace_symbol(symbol, new);
                }

                _ => {
                }
            }
        }

        self
    }

    pub fn to_generic(&self) -> Expression {
        Expression::List(self.clone())
    }

    pub fn make_generic(self) -> Expression {
        Expression::List(self)
    }
}

impl BaseExpression for SExpression {
    fn to_string(&self) -> String {
        let delimiter = ", ";
        let mut body = String::with_capacity(15);

        for (entry_number, entry) in self.expressions.iter().skip(1).enumerate() {
            body.push_str(&entry.as_str());

            if entry_number != (self.expressions.len() - 2) {
              body.push_str(delimiter);
            }
        }

        format!("{}[{}]", self.get_head().as_str(), body)
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }

    fn get_head(&self) -> &SimplexAtom {
        let head = self.expressions.front().unwrap();

        match head {
            &Expression::Atomic(ref x) => &x,
            _ => panic!("Invalid value passed...")
        }
    }
}
