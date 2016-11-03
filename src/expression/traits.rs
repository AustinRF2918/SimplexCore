extern crate decimal;
use decimal::d128;
use std::sync::{Arc, Mutex};

use atom::atom::SimplexAtom;

use expression::structure::Expression;
use expression::m_expression::structure::MExpression;
use expression::s_expression::structure::SExpression;

use std::borrow::Cow;

pub trait BaseExpression {
    fn get_head(&self) -> SimplexAtom;
    fn get_rest(&self) -> Expression;
    fn to_string(&self) -> String;
    fn as_str<'a>(&'a self) -> Cow<'a, str>;
}

pub trait Transmutable<T: BaseExpression> {
    fn get_internal_arc(&self) -> Option<Arc<Mutex<T>>>;
    fn transmute(&mut self, e: &Expression) -> Option<Arc<Mutex<T>>>;
    // Deep copies the internal data inside of an expression,
    // meaning that if the underlying data originally passed in
    // changes, this will not.
}
