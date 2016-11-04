use std::collections::LinkedList;

use expression::traits::BaseExpression;
use expression::structure::SimplexPointer;

use expression::atom::structure::SimplexAtom;
use parsing::utilities::symbols::representable_symbol;

// SExpression == SimplexList
#[derive(Clone, Debug)]
pub struct SimplexList {
    head: SimplexAtom,
    uniq_id: u64,
    expressions: LinkedList<SimplexPointer>,
}


impl Drop for SimplexList {
    fn drop(&mut self) {
        self.expressions.clear();
        println!("[Lightweight] Dropping List: {} with id: {}", self.as_str(), self.uniq_id());
    }
}

impl SimplexList {
    pub fn new(head_name: &str) -> SimplexList {
        if representable_symbol(head_name) {
            SimplexList {
                head: SimplexAtom::from(head_name),
                expressions: LinkedList::new(),
                uniq_id: 0
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

    pub fn pop(mut self) -> SimplexList {
        self.expressions.pop_back();
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

    fn get_rest(&self) -> Option<SimplexPointer> {
        println!("[get_rest] Begin..");
        let mut new_list = self.expressions.clone();
        new_list.pop_front();

        if new_list.len() == 0 {
            None
        } else {
            Some(SimplexPointer::from(SimplexList{head: SimplexAtom::from("List"), uniq_id: 0, expressions: new_list}))
        }

    }

    fn to_string(&self) -> String {
        //Watch out!
        format!("{}[{}]", self.get_head().unwrap().as_str(), self.body_to_string())
    }

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        if self.head.as_str() == symbol.as_str() {
            self.head = SimplexAtom::from(new.to_string());
        }

        for i in &mut self.expressions {
            i.replace_symbol(symbol, new);
        }

        SimplexPointer::from(self.clone())
    }

    fn uniq_id(&self) -> String {
        self.uniq_id.to_string()
    }

    fn set_uniq_id(&mut self, id: u64) {
        self.uniq_id = id;
    }
}
