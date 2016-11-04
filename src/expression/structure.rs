use std::mem;
use std::borrow::Cow;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

use expression::traits::BaseExpression;
use expression::list::structure::SimplexList;
use expression::atom::structure::SimplexAtom;

#[derive(Clone)]
pub struct SimplexPointer {
    internal_data: Arc<Mutex<BaseExpression>>
}

impl fmt::Debug for SimplexPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.internal_data.lock().unwrap().as_str()))
    }
}

impl Eq for SimplexPointer {
}

impl PartialEq for SimplexPointer{
    fn eq(&self, other: &Self) -> bool {
        // Deadlock risk.
        (*(self.internal_data.lock().unwrap().to_string())) == (*(other.internal_data.lock().unwrap().to_string()))
    }
}

impl BaseExpression for SimplexPointer {
    fn get_head(&self) -> Option<SimplexAtom> {
        // Fix unwrap anti-pattern.
        let lock = self.internal_data.lock().unwrap();
        lock.get_head()
    }

    fn get_rest(&self) -> Option<SimplexPointer> {
        // Fix unwrap anti-pattern.
        let lock = self.internal_data.lock().unwrap();
        match lock.get_rest() {
            Some(data) => {
                Some(SimplexPointer {
                    internal_data: Arc::new(Mutex::new(data))
                })
            }

            None => {
                None
            }
        }
    }

    fn to_string(&self) -> String {
        let lock = self.internal_data.lock().unwrap();
        lock.to_string()
    }

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        let mut lock = self.internal_data.lock().unwrap();
        lock.replace_symbol(symbol, new);
        self.clone()
    }
}

impl<'a> From<&'a str> for SimplexPointer {
    fn from(s: &str) -> SimplexPointer {
        SimplexPointer {
            internal_data: Arc::new(Mutex::new(SimplexAtom::from(s)))
        }
    }
}

impl From<SimplexAtom> for SimplexPointer {
    fn from(a: SimplexAtom) -> SimplexPointer {
        SimplexPointer {
            internal_data: Arc::new(Mutex::new(a))
        }
    }
}

impl From<SimplexList> for SimplexPointer {
    fn from(s: SimplexList) -> SimplexPointer {
        SimplexPointer {
            internal_data: Arc::new(Mutex::new(s))
        }
    }
}

