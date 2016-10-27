use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::Expression;

use atom::atom::SimplexAtom;


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

    pub fn push_expression(&mut self, e: Expression) {
        self.expressions.push_back(e);
    }
}

impl BaseExpression for SExpression {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(15);
        let mut flag = false;
        s.push_str("List[");
        for entry in &self.expressions {
            if flag == true {
                s.push_str(entry.to_string().as_str());
                s.push(',');
                s.push(' ');
            } else {
                flag = true;
            }
        }

        if self.expressions.len() != 1 {
            s.pop();
            s.pop();
        }
        s.push(']');
        s
    }

    fn get_head(&self) -> &SimplexAtom {
        let head = self.expressions.front().unwrap();

        match head {
            &Expression::Atomic(ref x) => &x,
            _ => panic!("Invalid value passed...")
        }
    }
}
