extern crate decimal;
use decimal::d128;
use std::sync::{Arc, Mutex};

use atom::atom::SimplexAtom;

use expression::structure::Expression;
use expression::m_expression::structure::MExpression;
use expression::s_expression::structure::SExpression;
use std::borrow::Cow;

pub trait BaseExpression {
    fn get_head(&self) -> Option<SimplexAtom>;
    fn get_rest(&self) -> Option<Self> where Self: Sized;
    fn to_string(&self) -> String;
    fn as_str<'a>(&'a self) -> Cow<'a, str> { Cow::Owned(self.to_string())} 
}

// Great API for get_rest:
// Make it generic across itself: Meaning an SExpression returns an SExpression, MExpression...
// That idea is that with an SExpression, it is a simplex copy pop front, MExpression deletes
// parameters, and then deletes constants, atom deletes its own head and then gives the next
// data.. Optional as well would be smart.

pub trait Transmutable<T: BaseExpression> {
    fn get_internal_arc(&self) -> Option<Arc<Mutex<T>>>;
    fn transmute(&mut self, e: &Expression) -> Option<Arc<Mutex<T>>>;
    // Deep copies the internal data inside of an expression,
    // meaning that if the underlying data originally passed in
    // changes, this will not.
}
