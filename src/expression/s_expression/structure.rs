use std::borrow::Cow;

use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::Expression;

use atom::atom::SimplexAtom;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SExpression {
    head: SimplexAtom,
    expressions: LinkedList<Expression>,
}

impl SExpression {
    pub fn new() -> SExpression {
        SExpression {
            head: SimplexAtom::from("List"),
            expressions: LinkedList::new(),
        }
    }

    pub fn push_expression(mut self, e: Expression) -> SExpression {
        self.expressions.push_back(e);
        self
    }

    pub fn replace_symbol(mut self, symbol: &Expression, new: &Expression) -> SExpression {
        match &mut self.head {
            &mut SimplexAtom::SimplexSymbol(_) => {
                if self.head.as_str() == symbol.as_str() {
                    let atom_string = new.to_string();
                    self.head = SimplexAtom::from(atom_string.as_str());
                }
            }
            _ => {
                panic!("Somehow a non-primitive was part of the SExpression head...");
            }
        }

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

    pub fn body_to_string(&self) -> String {
        let delimiter = ", ";
        let mut body = String::with_capacity(self.expressions.len() * 5);

        for (entry_number, entry) in self.expressions.iter().enumerate() {
            body.push_str(&entry.as_str());

            if entry_number != (self.expressions.len() - 1) {
              body.push_str(delimiter);
            }
        }

        body
    }
}

impl BaseExpression for SExpression {
    fn to_string(&self) -> String {
        format!("{}[{}]", self.get_head().as_str(), self.body_to_string())
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }

    fn get_head(&self) -> &SimplexAtom {
        &self.head
    }
}
