extern crate decimal;
use decimal::d128;

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

pub trait Transmutable {
    // Deep copies the internal data inside of an expression,
    // meaning that if the underlying data originally passed in
    // changes, this will not.
    fn transmute_deep(&self, e: &Expression) -> Expression;

    // Copies the pointer value stored in the Arc to an expression.
    // This means that if we change something it will change
    // as well.
    fn transmute_shallow(&self, e: &Expression) -> Expression;
}
