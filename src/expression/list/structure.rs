use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::SimplexPointer;

use expression::atom::structure::SimplexAtom;
use parsing::utilities::symbols::representable_symbol;

// SExpression == SimplexList
#[derive(Clone, Debug)]
pub struct SimplexList {
    pub head: SimplexAtom,
    expressions: LinkedList<SimplexPointer>,
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

    pub fn push(mut self, e: &SimplexPointer) -> SimplexList {
        self.expressions.push_back(e.clone());
        self
    }

    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    pub fn pop_back(mut self) -> SimplexList {
        self.expressions.pop_back();
        self
    }

    pub fn pop_front(mut self) -> SimplexList {
        self.expressions.pop_front();
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
    fn get_head(&self) -> Option<SimplexPointer> {
        Some(SimplexPointer::from(self.head.clone()))
    }

    fn get_rest(&self) -> Option<SimplexPointer> {
        println!("[Get Rest] Begin..");
        let mut new_list = self.expressions.clone();
        new_list.pop_front();

        if new_list.len() == 0 {
            None
        } else {
            Some(SimplexPointer::from(SimplexList{head: SimplexAtom::from("List"), expressions: new_list}))
        }

    }

    fn to_string(&self) -> String {
        //Watch out!
        format!("{}[{}]", self.get_head().unwrap().as_str(), self.body_to_string())
    }

    fn replace_symbol(&self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        SimplexPointer::from(
            SimplexList {
                head: self.head.clone(),
                expressions: self.expressions.iter().map(|x| x.replace_symbol(symbol, new)).collect::<LinkedList<SimplexPointer>>(),
            }
        )
    }

    fn evaluate(&self, v: &Vec<SimplexPointer>) -> SimplexPointer {
        let mut new_list = self.clone();

        for i in &mut new_list.expressions {
            let mut changed = i.clone();
            changed = changed.evaluate(v);
            *i = changed;
        }

        SimplexPointer::from(new_list)
    }
}
