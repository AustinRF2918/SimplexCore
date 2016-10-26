extern crate decimal;
use decimal::d128;
use atom::atom::SimplexAtom;
use expression::structure::Expression;

pub trait BaseExpression {

    fn get_head_name(&self) -> &str;
    fn get_head(&self) -> &SimplexAtom;
    fn get_expression_type(&self) -> &str;
    fn to_string(&self) -> String;
}

pub trait SExpression 
{
    fn get_leaves(&self) -> &Vec<Expression>;
}

pub trait SExpressionFrom<T>
    where T: BaseExpression
{
    fn push_leave(&mut self, leave: T);
}

pub trait SExpressionTo<T>
    where T: BaseExpression
{
    fn eval(&self) -> Option<T>;
}

pub trait MetaExpression<T>
    where T: BaseExpression
{
    fn get_meta_variables(&self) -> &Vec<T>;
    fn push_meta_variable(&mut self, leave: T);
    fn check_meta_variables(&mut self, m_vars: Vec<T>) -> bool;
    fn m_eval(&self, m_vars: Vec<T>) -> SExpression;
}

// pub trait ComposedMExpression<T: BaseMExpression>{
// fn get_leaves(&self) -> Vec<T>;
// fn push_leave(&mut self, T);
// fn get_m_variables(&self) -> Vec<T>;
// fn push_m_variable(&mut self, T) -> Vec<T>;
// }
//

// pub trait ExpressionSequencable<T: BaseExpression>{
// fn sequences(self) -> Option<Vec<T>>;
// fn flatten(self) -> Self;
// fn flatten_sequence(self) -> Self;
// fn flatten_pattern_sequence(self) -> Self;
// }
//
