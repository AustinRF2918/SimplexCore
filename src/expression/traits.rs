extern crate decimal;
use decimal::d128;
use atom::atom::SimplexAtom;
use expression::structure::Expression;
use std::borrow::Cow;

pub trait BaseExpression {
    fn get_head(&self) -> &SimplexAtom;
    fn to_string(&self) -> String;
    fn as_str<'a>(&'a self) -> Cow<'a, str>;
}
