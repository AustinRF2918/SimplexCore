use std::mem;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use atom::atom::SimplexAtom;

use expression::traits::{BaseExpression, Transmutable};
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
        Expression::Atomic(a.clone())
    }
}

impl From<Arc<Mutex<SExpression>>> for Expression {
    fn from(s: Arc<Mutex<SExpression>>) -> Expression {
        Expression::List(s.clone())
    }
}


impl Transmutable<SExpression> for Expression{
    fn get_internal_arc(&self) -> Option<Arc<Mutex<SExpression>>> {
        match self {
            &Expression::List(ref x) => {
                Some(x.clone())
            }
            &Expression::Atomic(_) => {
                None
            }
        }
    }

    fn transmute(&mut self, e: &Expression) -> Option<Arc<Mutex<SExpression>>> {
        let internal_arc : Option<Arc<Mutex<SExpression>>> = e.get_internal_arc();
        match *self {
            Expression::List(x) => {
                x = internal_arc.unwrap();
                Some(x)
            }
            _ => {
                None
            }
        }
    }
}

impl Transmutable<SimplexAtom> for Expression{
    fn get_internal_arc(&self) -> Option<Arc<Mutex<SimplexAtom>>> {
        match self {
            &Expression::List(_) => {
                None
            }
            &Expression::Atomic(ref x) => {
                Some(x.clone())
            }
        }
    }

    fn transmute(&mut self, e: &Expression) -> Option<Arc<Mutex<SimplexAtom>>> {
        let internal_arc : Option<Arc<Mutex<SimplexAtom>>> = e.get_internal_arc();
        match *self {
            Expression::Atomic(ref mut x) => {
                let z = internal_arc;
                x = &mut z.unwrap();
                Some(x.clone())
            }
            _ => {
                None
            }
        }
    }
}
