extern crate decimal;
use decimal::d128;
use std::sync::{Arc, Mutex};

use atom::atom::SimplexAtom;
use std::borrow::Cow;

use expression::structure::ExpressionPointer;

pub trait BaseExpression {
    fn get_head(&self) -> Option<SimplexAtom>;
    fn get_rest(&self) -> Option<ExpressionPointer>;
    fn to_string(&self) -> String;
    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> ExpressionPointer;
    fn as_str<'a>(&'a self) -> Cow<'a, str> { Cow::Owned(self.to_string())} 
}
