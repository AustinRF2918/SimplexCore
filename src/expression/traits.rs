use std::borrow::Cow;

use expression::structure::SimplexPointer;
use expression::atom::structure::SimplexAtom;

pub trait BaseExpression {
    fn get_head(&self) -> Option<SimplexAtom>;
    fn get_rest(&self) -> Option<SimplexPointer>;

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer;

    fn to_string(&self) -> String;
    fn as_str<'a>(&'a self) -> Cow<'a, str> { Cow::Owned(self.to_string())} 
}
