use std::fmt;
use std::sync::{Arc, Mutex};

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
        // Note that I believe that this schema is in general capable of deadlocking:
        // here is the rational behind it:
        // ExpressionA: SimplexPointer , ExpressionB: SimplexPointer
        // 
        // Let's say that ExpressionA checks for equality on ExpressionB on one thread,
        // and that the opposite case occurs on another thread.
        //
        // Thread1:                Thread2:
        // ExpressionA.lock()      ExpressionB.lock()
        // ExpressionB.lock()      ExpressionA.lock()
        // waiting on B            waiting on A
        //
        // For this reason, it may be a better idea instead of using a Mutex to use a
        // RWCell.

        let lock_lhs = self.internal_data.lock();
        let lock_rhs = other.internal_data.lock();

        match (lock_lhs, lock_rhs) {
            (Ok(guard_lhs), Ok(guard_rhs)) => {
                *guard_lhs.to_string() == *guard_rhs.to_string()
            }

            (Err(poisoned_lhs), Ok(guard_rhs)) => {
                *(poisoned_lhs.into_inner()).to_string() == *guard_rhs.to_string()
            }

            (Ok(guard_lhs), Err(poisoned_rhs)) => {
                *guard_lhs.to_string() == *(poisoned_rhs.into_inner()).to_string()
            }

            (Err(poisoned_lhs), Err(poisoned_rhs)) => {
                *(poisoned_lhs.into_inner()).to_string() == *(poisoned_rhs.into_inner()).to_string()
            }
        }
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

