use std::mem;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use atom::atom::SimplexAtom;

use expression::traits::{BaseExpression, Transmutable};
use expression::list::structure::SimplexList;

#[derive(Clone, Debug)]
pub struct ExpressionPointer<T: BaseExpression> {
    internal_data: Arc<Mutex<T>>
}

impl BaseExpression for ExpressionPointer<SimplexList> {
    fn get_head(&self) -> Option<SimplexAtom> {
        // Fix unwrap anti-pattern.
        let lock = self.internal_data.lock().unwrap();
        lock.get_head()
    }

    fn get_rest(&self) -> Option<ExpressionPointer<SimplexList>> {
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
}

#[derive(Clone, Debug)]
pub enum Expression {
    List(SimplexList),
    Atomic(SimplexAtom)
}

impl BaseExpression for Expression {
    fn get_head(&self) -> Option<SimplexAtom> {
        match self {
            &Expression::List(ref internal) => {
                internal.get_head()
            }

            &Expression::Atomic(ref internal) => {
                internal.get_head()
            }
        }
    }

    fn get_rest(&self) -> Option<Expression> {
        match self {
            &Expression::List(ref internal) => {
                match internal.get_rest() {
                    Some(data) => {
                        Some(Expression::from(data))
                    }

                    None => {
                        None
                    }
                }
            }

            &Expression::Atomic(ref internal) => {
                match internal.get_rest() {
                    Some(data) => {
                        Some(Expression::from(data))
                    }

                    None => {
                        None
                    }
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            &Expression::List(ref internal) => {
                internal.to_string()
            }

            &Expression::Atomic(ref internal) => {
                internal.to_string()
            }
        }
    }
}

impl<'a> From<&'a str> for Expression {
    fn from(s: &str) -> Expression {
        Expression::Atomic(SimplexAtom::from(s))
    }
}

impl From<SimplexAtom> for Expression {
    fn from(a: SimplexAtom) -> Expression {
        Expression::Atomic(a)
    }
}

impl From<SimplexList> for Expression {
    fn from(s: SimplexList) -> Expression {
        Expression::List(s)
    }
}
