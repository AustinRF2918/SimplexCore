use atom::atom::SimplexAtom;

use arithmetic::plus::Plus;
use arithmetic::subtract::Subtract;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression {
    Add(Plus),
    Sub(Subtract),
    Atomic(SimplexAtom)
}
