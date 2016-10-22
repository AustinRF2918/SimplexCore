use atom::atom::SimplexAtom;
use atom::traits;
use parsing::utilities::string::representable_string;
use parsing::utilities::numerics::representable_numeric;

use expression::structures::integrity::checks::ensure_context;
use expression::structures::attributes::BaseExpression;
use expression::structures::attributes::SymbolicExpression;
use expression::structures::attributes::MetaExpression;

use atom::numbers::number::Numeric;
use atom::symbols::symbol::Symbol;
use atom::strings::string::SString;

extern crate decimal;
use decimal::d128;

/// BaseExpression
///
/// This is the structure that we use to represent a
/// base expression in Simplex: It consists of a pattern
/// sequence, isFormatted, represented by a boolinear which
/// will tell us we have a self referencing expression, and
/// last evaluated, which contains the last evaluation performed
/// in a linear evaluation.
#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Plus {
    head: SimplexAtom,
    leaves: Vec<SimplexAtom>,
}

impl Plus {
    fn new() -> Plus {
        Plus {
            head: SimplexAtom::SimplexSymbol(Symbol::from_str("Plus").unwrap()),
            leaves: Vec::new(),
        }
    }
}
impl BaseExpression<SimplexAtom> for Plus {
    fn get_expression_type(&self) -> &str {
        "Simplex`MExpression"
    }

    fn get_head_name(&self) -> &str {
        match self.head {
            SimplexAtom::SimplexNumeric(_) => "Simplex`Invalid",
            SimplexAtom::SimplexString(_) => "Simplex`Invalid",
            SimplexAtom::SimplexSymbol(ref s) => s.to_string().as_str(),
        }
    }

    fn get_head(&self) -> &SimplexAtom {
        &self.head
    }

    fn reduce_expression(self) -> Plus {
        self
    }

    fn get_int_value(&self) -> Option<i64> {
        None
    }

    fn get_float_value(&self) -> Option<d128> {
        None
    }

    fn get_string_value(&self) -> Option<&String> {
        None
    }
}

impl SymbolicExpression<SimplexAtom> for PrimitiveExpression {
    fn get_leaves(&self) -> &Vec<SimplexAtom> {
        &self.leaves
    }

    fn push_leave(&mut self, leave: SimplexAtom) {
        self.leaves.push(leave);
    }
}
