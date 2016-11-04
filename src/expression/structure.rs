use std::fmt;
use std::sync::{Arc, RwLock};

use expression::traits::BaseExpression;
use expression::list::structure::SimplexList;
use expression::atom::structure::SimplexAtom;

#[derive(Clone)]
pub struct SimplexPointer {
    internal_data: Arc<RwLock<BaseExpression>>,
    uniq_id: u64
}

impl Drop for SimplexPointer {
    fn drop(&mut self) {
        println!("[Lightweight] Dropping Pointer: {} with id: {}", self.as_str(), self.uniq_id());
    }
}

impl fmt::Debug for SimplexPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.internal_data.read() {
            Ok(guard) => {
                write!(f, "Write: {}", guard.as_str())
            }

            Err(poisoned) => {
                write!(f, "Poisoned: {}", poisoned.into_inner().as_str())
            }
        }
    }
}

impl Eq for SimplexPointer {
}

impl PartialEq for SimplexPointer{
    fn eq(&self, other: &Self) -> bool {
        // Note that I believe that this schema is in general capable of deadwriteing:
        // here is the rational behind it:
        // ExpressionA: SimplexPointer , ExpressionB: SimplexPointer
        // 
        // Let's say that ExpressionA checks for equality on ExpressionB on one thread,
        // and that the opposite case occurs on another thread.
        //
        // Thread1:                Thread2:
        // ExpressionA.write()      ExpressionB.write()
        // ExpressionB.write()      ExpressionA.write()
        // waiting on B            waiting on A
        //
        // For this reason, it may be a better idea instead of using a Mutex to use a
        // RWCell.

        let read_lhs = self.internal_data.read();
        let read_rhs = other.internal_data.read();

        match (read_lhs, read_rhs) {
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
        let write = self.internal_data.write().unwrap();
        write.get_head()
    }

    fn get_rest(&self) -> Option<SimplexPointer> {
        let read = self.internal_data.read().unwrap();
        match read.get_rest() {
            Some(data) => {
                Some(SimplexPointer {
                    internal_data: Arc::new(RwLock::new(data)),
                    uniq_id: 0

                })
            }

            None => {
                None
            }
        }
    }

    fn to_string(&self) -> String {
        let write = self.internal_data.read().unwrap();
        write.to_string()
    }

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        let mut write = self.internal_data.write();
        match write {
            Ok(mut lock) => {
                lock.replace_symbol(symbol, new);
            }
            Err(poisoned) => {
                poisoned.into_inner().replace_symbol(symbol, new);
            }
        }
        self.clone()
    }

    fn uniq_id(&self) -> String {
        self.uniq_id.to_string()
    }

    fn set_uniq_id(&mut self, id: u64) {
        self.uniq_id = id;
    }
}

impl<'a> From<&'a str> for SimplexPointer {
    fn from(s: &str) -> SimplexPointer {
        SimplexPointer {
            internal_data: Arc::new(RwLock::new(SimplexAtom::from(s))),
            uniq_id: 0,
        }
    }
}

impl From<SimplexAtom> for SimplexPointer {
    fn from(a: SimplexAtom) -> SimplexPointer {
        SimplexPointer {
            internal_data: Arc::new(RwLock::new(a)),
            uniq_id: 0,
        }
    }
}

impl From<SimplexList> for SimplexPointer {
    fn from(s: SimplexList) -> SimplexPointer {
        SimplexPointer {
            internal_data: Arc::new(RwLock::new(s)),
            uniq_id: 0,
        }
    }
}

