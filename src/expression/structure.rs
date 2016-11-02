use std::str::FromStr;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use atom::atom::SimplexAtom;

use expression::traits::BaseExpression;
use expression::s_expression::structure::SExpression;

#[derive(Clone, Debug)]
pub enum Expression {
    List(Arc<Mutex<SExpression>>),
    Atomic(Arc<Mutex<SimplexAtom>>)
}

impl Expression {
    pub fn new_primitive(s: &str) -> Expression {
        Expression::Atomic(Arc::new(Mutex::new(SimplexAtom::from(s))))
    }

    pub fn new_list(l: SExpression) -> Expression {
        Expression::List(Arc::new(Mutex::new(l)))
    }

    pub fn to_string(&self) -> String {
        match self {
            &Expression::List(ref list) => {
                let mut item = list.lock().unwrap();
                item.to_string().clone()
            }
            &Expression::Atomic(ref atom) => {
                // TODO: DONT UNWRAP
                let mut item = atom.lock().unwrap();
                item.to_string().clone()
            },
        }
    }

    pub fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }
}

impl BaseExpression for Expression {
    fn get_head(&self) -> SimplexAtom {
        match self {
            &Expression::List(ref internal) => {
                // FIX UNWRAP ANTI-PATTERN;
                let lock = internal.lock().unwrap();
                lock.get_head()
            }

            &Expression::Atomic(ref  internal) => {
                // FIX UNWRAP ANTI-PATTERN;
                let lock = internal.lock().unwrap();
                lock.get_head()
            }
        }
    }

    fn get_rest(&self) -> Expression {
        match self {
            &Expression::List(ref internal) => {
                // FIX UNWRAP ANTI-PATTERN;
                let lock = internal.lock().unwrap();
                lock.get_rest()
            }

            &Expression::Atomic(ref internal) => {
                // FIX UNWRAP ANTI-PATTERN;
                let lock = internal.lock().unwrap();
                lock.get_rest()
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            &Expression::List(ref internal) => {
                // FIX UNWRAP ANTI-PATTERN;
                let lock = internal.lock().unwrap();
                lock.to_string()
            }

            &Expression::Atomic(ref internal) => {
                // FIX UNWRAP ANTI-PATTERN;
                let lock = internal.lock().unwrap();
                lock.to_string()
            }
        }
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str>{
        Cow::Owned(self.to_string())
    }
}

impl<'a> From<&'a str> for Expression {
    fn from(s: &str) -> Expression {
        Expression::Atomic(Arc::new(Mutex::new(SimplexAtom::from(s))))
    }
}

impl From<SimplexAtom> for Expression {
    fn from(a: SimplexAtom) -> Expression {
        Expression::Atomic(Arc::new(Mutex::new(a)))
    }
}

impl From<SExpression> for Expression {
    fn from(s: SExpression) -> Expression {
        Expression::List(Arc::new(Mutex::new(s)))
    }
}

impl From<Arc<Mutex<SimplexAtom>>> for Expression {
    fn from(a: Arc<Mutex<SimplexAtom>>) -> Expression {
        Expression::Atomic(a)
    }
}

impl From<Arc<Mutex<SExpression>>> for Expression {
    fn from(s: Arc<Mutex<SExpression>>) -> Expression {
        Expression::List(s)
    }
}
