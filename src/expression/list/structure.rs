use std::borrow::Cow;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

use expression::traits::BaseExpression;
use expression::structure::ExpressionPointer;

use atom::atom::SimplexAtom;
use parsing::utilities::symbols::representable_symbol;

// SExpression == SimplexList
#[derive(Clone, Debug)]
pub struct SimplexList {
    head: SimplexAtom,
    expressions: LinkedList<ExpressionPointer>,
}

impl SimplexList {
    pub fn new(head_name: &str) -> SimplexList {
        if representable_symbol(head_name) {
            SimplexList {
                head: SimplexAtom::from(head_name),
                expressions: LinkedList::new(),
            }
        } else {
            // Implement Error Type 
            panic!("Bad head name passed to SimplexList.");
        }
    }

    pub fn push(mut self, e: &ExpressionPointer) -> SimplexList {
        self.expressions.push_back(e.clone());
        self
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

impl BaseExpression for SimplexList {
    fn get_head(&self) -> Option<SimplexAtom> {
        Some(self.head.clone())
    }

    fn get_rest(&self) -> Option<ExpressionPointer> {
        let mut new_list = self.clone();
        new_list.expressions.pop_front();

        if new_list.expressions.len() == 0 {
            None
        } else {
            Some(ExpressionPointer::from(new_list))
        }
    }

    fn to_string(&self) -> String {
        //Watch out!
        format!("{}[{}]", self.get_head().unwrap().as_str(), self.body_to_string())
    }

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> ExpressionPointer {
        if self.head.as_str() == symbol.as_str() {
            self.head = SimplexAtom::from(new.to_string());
        }

        for i in &mut self.expressions {
            // TODO: REIMPLEMENT LOGIC.
        }

        ExpressionPointer::from(self.clone())
    }
}
