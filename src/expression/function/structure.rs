use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::SimplexPointer;
use expression::list::structure::SimplexList;
use expression::atom::structure::SimplexAtom;

#[derive(Clone, Debug)]
pub struct SimplexFunction {
    head: SimplexAtom,
    uniq_id: u64,
    reflexive: bool,
    meta_variables: LinkedList<SimplexAtom>,
    s_expression: SimplexList,
}

impl SimplexFunction {
    pub fn new(head_name: &str) -> SimplexFunction {
        SimplexFunction {
            head: SimplexAtom::from(head_name),
            reflexive: false,
            meta_variables: LinkedList::new(),
            s_expression: SimplexList::new("List"),
            uniq_id: 0
        }
    }

    pub fn push(mut self, p: &SimplexPointer) -> SimplexFunction {
        let mut new_self = self.clone();
        new_self.push(&p.clone());
        self
    }

    pub fn push_meta_variable(mut self, a: SimplexAtom) -> SimplexFunction {
        self.meta_variables.push_back(a.clone());
        self
    }

    pub fn toggle_reflexive(mut self) -> SimplexFunction {
        self.reflexive = !self.reflexive;
        self
    }

    pub fn evaluate(&self, params: &Vec<&str>) -> SimplexList {
        let mut new_s_expression = self.s_expression.clone();

        if self.reflexive {
        }

        let mut new_params = params.clone();

        for (item_number, item) in self.meta_variables.iter().enumerate().rev() {
            match new_params.pop() {
                Some(thing) => {
                    new_s_expression.replace_symbol(&SimplexPointer::from(item.clone()), &SimplexPointer::from(thing));
                }

                None => {
                }
            }
        }

        new_s_expression
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
    fn get_head(&self) -> Option<SimplexAtom> {
        Some(self.head.clone())
    }

    fn get_rest(&self) -> Option<SimplexPointer> {
        // TODO: Implement rest schema as follows:
        // First remove meta variable and associated RHS entries.
        // Once RHS is removed, progressively remove RHS SimplexList
        // values via its own get_rest().

        Some(SimplexPointer::from(self.clone()))
    }

    fn to_string(&self) -> String {
        format!("{}[{}] := {}", self.get_head().unwrap().as_str(), self.m_vars_to_string(), self.s_expression.to_string())
    }

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        SimplexPointer::from(self.clone())
    }

    fn uniq_id(&self) -> String {
        self.uniq_id.to_string()
    }

    fn set_uniq_id(&mut self, id: u64) {
        self.uniq_id = id;
    }
}
