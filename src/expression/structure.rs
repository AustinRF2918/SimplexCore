use std::fmt;
use std::sync::{Arc, RwLock};
use std::cell::RefCell;
use std::rc::Rc;

use expression::traits::BaseExpression;
use expression::list::structure::SimplexList;
use expression::function::structure::SimplexFunction;
use expression::atom::structure::SimplexAtom;

#[derive(Clone)]
pub struct SimplexPointer {
    internal_data: Rc<RefCell<Box<BaseExpression>>>,
}

impl SimplexPointer {
    fn new(e: Box<BaseExpression>) -> SimplexPointer {
        SimplexPointer {
            internal_data: Rc::new(RefCell::new(e))
        }
    }
}

impl fmt::Debug for SimplexPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimplexPointer: {}", (*self.internal_data.borrow()).as_str())
    }
}

impl fmt::Display for SimplexPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (*self.internal_data.borrow()).as_str())
    }
}

impl Eq for SimplexPointer {
}

impl PartialEq for SimplexPointer{
    fn eq(&self, other: &Self) -> bool {
        *self.internal_data.borrow().to_string() == *other.internal_data.borrow().to_string()
    }
}

impl BaseExpression for SimplexPointer {
    fn get_head(&self) -> Option<SimplexPointer> {
        (*self.internal_data.borrow()).get_head()
    }

    fn get_rest(&self) -> Option<SimplexPointer> {
        (*self.internal_data.borrow()).get_rest()
    }

    fn to_string(&self) -> String {
        (*self.internal_data.borrow()).to_string()
    }

    fn replace_symbol(&self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        (*self.internal_data.borrow()).replace_symbol(symbol, new)
    }

    fn evaluate(&self, v: &Vec<SimplexPointer>) -> SimplexPointer {
        (*self.internal_data.borrow()).evaluate(v)
    }
}

impl<'a> From<&'a str> for SimplexPointer {
    fn from(s: &str) -> SimplexPointer {
        SimplexPointer {
            internal_data: Rc::new(RefCell::new(Box::new(SimplexAtom::from(s)))),
        }
    }
}

impl From<String> for SimplexPointer {
    fn from(s: String) -> SimplexPointer {
        SimplexPointer {
            internal_data: Rc::new(RefCell::new(Box::new(SimplexAtom::from(s)))),
        }
    }
}

impl From<SimplexAtom> for SimplexPointer {
    fn from(a: SimplexAtom) -> SimplexPointer {
        SimplexPointer {
            internal_data: Rc::new(RefCell::new(Box::new(a))),
        }
    }
}

impl From<SimplexList> for SimplexPointer {
    fn from(s: SimplexList) -> SimplexPointer {
        SimplexPointer {
            internal_data: Rc::new(RefCell::new(Box::new(s))),
        }
    }
}

impl From<SimplexFunction> for SimplexPointer {
    fn from(s: SimplexFunction) -> SimplexPointer {
        SimplexPointer {
            internal_data: Rc::new(RefCell::new(Box::new(s))),
        }
    }
}
