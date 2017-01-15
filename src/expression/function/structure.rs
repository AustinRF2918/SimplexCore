use std::fmt;

use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::SimplexPointer;
use expression::list::structure::SimplexList;
use expression::atom::structure::SimplexAtom;

#[derive(Clone)]
pub struct SimplexFunction {
    head: SimplexAtom,
    meta_variables: LinkedList<SimplexAtom>,
    s_expression: SimplexList,
    base_evaluation: bool
}

impl fmt::Debug for SimplexFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: REIMPLMENT WITH S_EXPRESSION FOR META_VARIABLES.
        write!(f, "SimplexFunction: {:?}", self.head)
    }
}

impl SimplexFunction {
    pub fn new(head_name: &str) -> SimplexFunction {
        SimplexFunction {
            head: SimplexAtom::from(head_name),
            meta_variables: LinkedList::new(),
            s_expression: SimplexList::new("List"),
            base_evaluation: false,
        }
    }

    pub fn push(mut self, p: &SimplexPointer) -> SimplexFunction {
        self.s_expression  = self.s_expression.push(&p.clone());
        self
    }

    pub fn push_meta_variable(mut self, a: SimplexAtom) -> SimplexFunction {
        self.meta_variables.push_back(a.clone());
        self
    }

    pub fn toggle_base_evaluation(&mut self) {
        self.base_evaluation = !self.base_evaluation;
        if self.base_evaluation {
            self.s_expression.head = SimplexAtom::from(self.head.to_string());
        } else {
            self.s_expression.head = SimplexAtom::from("List");
        }
    }

    pub fn m_vars_to_string(&self) -> String {
        let delimiter = ", ";
        let mut body = String::with_capacity(self.meta_variables.len() * 5);

        for (entry_number, entry) in self.meta_variables.iter().enumerate() {
            body.push_str(&entry.as_str());
            body.push('_');

            if entry_number != (self.meta_variables.len() - 1) {
              body.push_str(delimiter);
            }
        }

        body
    }
}

impl BaseExpression for SimplexFunction {
    fn get_head(&self) -> Option<SimplexPointer> {
        Some(SimplexPointer::from(self.head.clone()))
    }

    fn get_rest(&self) -> Option<SimplexPointer> {
        let mut new_fn = self.clone();

        // Watch out for this logic.
        if self.meta_variables.len() != 0 {
            new_fn.meta_variables.pop_front();
            Some(SimplexPointer::from(new_fn))
        } else if self.s_expression.len() != 1 {
            new_fn.s_expression = new_fn.s_expression.pop_front();
            Some(SimplexPointer::from(new_fn))
        } else {
            Some(SimplexPointer::from(self.get_head().unwrap().to_string()))
        }
    }

    fn to_string(&self) -> String {
        format!("{}[{}] := {}", self.get_head().unwrap().as_str(), self.m_vars_to_string(), self.s_expression.to_string())
    }

    fn replace_symbol(&self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        let new = self.s_expression.replace_symbol(symbol, new);
        SimplexPointer::from(new)
    }

    fn evaluate(&self, v: &Vec<SimplexPointer>) -> SimplexPointer {
        println!("Eval_Before: {}", self.as_str());
        let mut new_s_expression = SimplexPointer::from(self.s_expression.clone());
        let mut new_params = v.clone();

        for item in self.meta_variables.iter().rev() {
            match new_params.pop() {
                Some(thing) => {
                    println!("Parsing Param: {}", item.as_str());
                    println!("Parsing To: {}", thing.as_str());
                    new_s_expression = new_s_expression.replace_symbol(&SimplexPointer::from(item.clone()), &SimplexPointer::from(thing.clone()));
                    println!("Eval_After Op: {}", new_s_expression.as_str());
                }

                _ => {}
            }
        }

        println!("Eval_After: {}", new_s_expression.as_str());
        new_s_expression
    }
}
