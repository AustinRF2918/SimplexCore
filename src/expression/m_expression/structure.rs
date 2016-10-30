use std::borrow::Cow;

use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::Expression;
use expression::s_expression::structure::SExpression;

use atom::atom::SimplexAtom;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MExpression {
    head: SimplexAtom,
    meta_variables: LinkedList<SimplexAtom>,
    s_expression: SExpression
}

impl MExpression {
    pub fn new(head_name: &str) -> MExpression {
        MExpression {
            head: SimplexAtom::from(head_name),
            meta_variables: LinkedList::new(),
            s_expression:SExpression::new() 
        }
    }

    pub fn push_expression(mut self, e: Expression) -> MExpression {
        self.s_expression = self.s_expression.push_expression(e);
        self
    }

    pub fn push_meta_variable(mut self, e: Expression) -> MExpression {
        match e {
            Expression::List(_) => {panic!("You attempted to place a non-atomic as a meta variable!")}
            Expression::Atomic(atom) => {
                match atom {
                    SimplexAtom::SimplexSymbol(_) => {
                        self.meta_variables.push_back(atom);
                    }
                    _ => {
                        panic!("You attempted to place a number or string as a meta-variable into a meta-expression.");
                    }
                }
            }
        }
        self
    }

    pub fn evaluate(&self, params: Vec<&str>) -> SExpression {
        let mut new_s_expression = self.s_expression.clone();
        let mut new_params = params.clone();

        for (item_number, item) in self.meta_variables.iter().enumerate().rev() {
            match new_params.pop() {
                Some(thing) => {
                    new_s_expression = new_s_expression.replace_symbol(&Expression::from(item.clone()), &Expression::from(thing));
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

impl BaseExpression for MExpression {
    fn to_string(&self) -> String {
        format!("{}[{}] := {}", self.get_head().as_str(), self.m_vars_to_string(), self.s_expression.to_string())
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }

    fn get_head(&self) -> &SimplexAtom {
        &self.head
    }
}

