extern crate decimal;
use decimal::d128;

pub trait BaseExpression {
    fn get_expression_name(&self) -> &str;
    fn get_head_name(&self) -> &str;
}

pub trait PrimitiveConverter {
    fn get_int_value(&self) -> Option<i64>;
    fn get_float_value(&self) -> Option<d128>;
    fn get_string_value(&self) -> Option<&String>;
}

pub trait ComposableExpression<T: BaseExpression>{
    fn get_head(&self) -> Option<T>;
    fn get_leaves(&self) -> Vec<T>;
}

pub trait ExpressionSequencable<T: BaseExpression>{
    fn sequences(self) -> Option<Vec<T>>;
    fn flatten(self) -> Self;
    fn flatten_sequence(self) -> Self;
    fn flatten_pattern_sequence(self) -> Self;
}
