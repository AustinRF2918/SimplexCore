use std::collections::LinkedList;
use expression::traits::BaseExpression;
use expression::structure::Expression;


pub struct SExpression {
    expressions: LinkedList<Expression>
}

impl SExpression {
    pub fn new() -> SExpression {
        SExpression {
            expressions: LinkedList::new()
        }
    }

    pub fn add_expression(&mut self, e: SExpression) {
        self.expressions.push_back(e);
    }
}
