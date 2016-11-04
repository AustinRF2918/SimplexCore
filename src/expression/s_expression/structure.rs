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

    pub fn push_pointer(mut self, e: &Expression) -> SExpression {
        self.expressions.push_back(e.clone());
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

                &mut Expression::List(ref mut list) => {
                    *list = list.clone().replace_symbol(symbol.clone(), new.clone());
                }
            }
        }

        self
    }

    pub fn to_generic(&self) -> Expression {
        Expression::List(self.clone())
    }

    pub fn make_generic(self) -> Expression {
        Expression::List(self.clone())
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
    fn get_head(&self) -> Option<SimplexAtom> {
        Some(self.head.clone())
    }

    fn get_rest(&self) -> Option<SExpression> {
        let mut new_list = self.clone();
        new_list.expressions.pop_front();

        if new_list.expressions.len() == 0 {
            None
        } else {
            Some(new_list)
        }
    }

    fn to_string(&self) -> String {
        //Watch out!
        format!("{}[{}]", self.get_head().unwrap().as_str(), self.body_to_string())
    }
}
