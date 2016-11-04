use std::mem;
use std::borrow::Cow;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

use atom::atom::SimplexAtom;

use expression::traits::{BaseExpression, Transmutable};
use expression::list::structure::SimplexList;

#[derive(Clone)]
pub struct ExpressionPointer {
    internal_data: Arc<Mutex<BaseExpression>>
}

impl fmt::Debug for ExpressionPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.internal_data.lock().unwrap().as_str()))
    }
}

impl Eq for ExpressionPointer {
}

impl PartialEq for ExpressionPointer{
    fn eq(&self, other: &Self) -> bool {
        // Deadlock risk.
        (*(self.internal_data.lock().unwrap().to_string())) == (*(other.internal_data.lock().unwrap().to_string()))
    }
}

impl BaseExpression for ExpressionPointer {
    fn get_head(&self) -> Option<SimplexAtom> {
        // Fix unwrap anti-pattern.
        let lock = self.internal_data.lock().unwrap();
        lock.get_head()
    }

    fn get_rest(&self) -> Option<ExpressionPointer> {
        // Fix unwrap anti-pattern.
        let lock = self.internal_data.lock().unwrap();
        match lock.get_rest() {
            Some(data) => {
                Some(ExpressionPointer {
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

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> ExpressionPointer {
        let mut lock = self.internal_data.lock().unwrap();
        lock.replace_symbol(symbol, new);
        self.clone()
    }
}

impl<'a> From<&'a str> for ExpressionPointer {
    fn from(s: &str) -> ExpressionPointer {
        ExpressionPointer {
            internal_data: Arc::new(Mutex::new(SimplexAtom::from(s)))
        }
    }
}

impl From<SimplexAtom> for ExpressionPointer {
    fn from(a: SimplexAtom) -> ExpressionPointer {
        ExpressionPointer {
            internal_data: Arc::new(Mutex::new(a))
        }
    }
}

impl From<SimplexList> for ExpressionPointer {
    fn from(s: SimplexList) -> ExpressionPointer {
        ExpressionPointer {
            internal_data: Arc::new(Mutex::new(s))
        }
    }
}
