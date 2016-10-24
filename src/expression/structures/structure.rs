use atom::atom::SimplexAtom;
use expression::structures::builtins::plus::Plus;
use expression::structures::builtins::subtract::Subtract;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression {
    Add(Plus),
    Sub(Subtract),
    Atomic(SimplexAtom)
}


