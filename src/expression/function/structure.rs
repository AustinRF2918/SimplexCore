use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::Expression;
use expression::list::structure::SimplexList;

use atom::atom::SimplexAtom;

#[derive(Clone, Debug)]
pub struct SimplexFunction {
    head: SimplexAtom,
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
            s_expression: SimplexList::new("List") 
        }
    }

    pub fn push_expression(mut self, e: Expression) -> SimplexFunction {
        self.s_expression = self.s_expression.push_expression(e);
        self
    }

    pub fn push_meta_variable(mut self, e: Expression) -> SimplexFunction {
        match e {
            Expression::List(_) => {panic!("You attempted to place a non-atomic as a meta variable!")}
            Expression::Atomic(atom) => {
                self.meta_variables.push_back(atom.clone());
            }
        }
        self
    }

    pub fn toggle_reflexive(mut self) -> SimplexFunction {
        self.reflexive = !self.reflexive;
        self
    }

    pub fn evaluate(&self, params: &Vec<&str>) -> SimplexList {
        let mut new_s_expression = self.s_expression.clone();

        if self.reflexive {
            // ANTI PATTERN
            let head_name = self.get_head().unwrap().to_string();
            new_s_expression = new_s_expression.replace_symbol(Expression::from("List"), Expression::from(head_name.as_str()));
        }

        let mut new_params = params.clone();

        for (item_number, item) in self.meta_variables.iter().enumerate().rev() {
            match new_params.pop() {
                Some(thing) => {
                    new_s_expression = new_s_expression.replace_symbol(Expression::from(item.clone()), Expression::from(thing));
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

    fn get_rest(&self) -> Option<SimplexFunction> {
        // TODO: Implement rest schema as follows:
        // First remove meta variable and associated RHS entries.
        // Once RHS is removed, progressively remove RHS SimplexList
        // values via its own get_rest().

        Some(self.clone())
    }

    fn to_string(&self) -> String {
        format!("{}[{}] := {}", self.get_head().unwrap().as_str(), self.m_vars_to_string(), self.s_expression.to_string())
    }
}

