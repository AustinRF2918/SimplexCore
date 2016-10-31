use std::borrow::Cow;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

use expression::traits::BaseExpression;
use expression::structure::Expression;

use atom::atom::SimplexAtom;

use parsing::utilities::symbols::representable_symbol;

#[derive(Clone, Debug)]
pub struct SExpression {
    head: SimplexAtom,
    expressions: LinkedList<Expression>,
}

impl SExpression {
    pub fn new(head_name: &str) -> SExpression {
        if representable_symbol(head_name) {
            SExpression {
                head: SimplexAtom::from(head_name),
                expressions: LinkedList::new(),
            }
        } else {
            // Implement Error Type 
            panic!("Bad head name passed to SExpression.");
        }
    }

    pub fn push_expression(mut self, e: Expression) -> SExpression {
        self.expressions.push_back(e);
        self
    }

    pub fn replace_symbol(mut self, symbol: Expression, new: Expression) -> SExpression {
        match &mut self.head {
            &mut SimplexAtom::SimplexSymbol(_) => {
                if self.head.as_str() == symbol.as_str() {
                    // TODO: Make static replacement possible by making trait
                    // in expression non-cow. This would be considered an
                    // optimization and will not be done for a while.
                    self.head = SimplexAtom::from(new.to_string());
                }
            }
            _ => {
                panic!("Somehow a non-primitive was part of the SExpression head...");
            }
        }

        for i in &mut self.expressions {
            match i {
                &mut Expression::Atomic(_) => {
                    if i.to_string() == symbol.to_string() {
                        *i = new.clone();
                    }
                }

                &mut Expression::List(ref list) => {
                    let mut x = list.clone();
                    let mut lock = x.lock().unwrap();
                    *lock = lock.clone().replace_symbol(symbol.clone(), new.clone());
                }
            }
        }

        self
    }

    pub fn to_generic(&self) -> Expression {
        Expression::List(Arc::new(Mutex::new(self.clone())))
    }

    pub fn make_generic(self) -> Expression {
        Expression::List(Arc::new(Mutex::new(self.clone())))
    }

    pub fn body_to_string(&self) -> String {
        // TODO: This could be really nicely optimized by giving the BaseExpression trait
        // that ability to calculate its own length and use that instead of an approximation.

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

    fn get_head(&self) -> SimplexAtom {
        self.head.clone()
    }
}
