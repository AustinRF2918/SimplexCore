use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use atom::numbers::number::Numeric;
use atom::numbers::traits;

use expression::traits::BaseExpression;
use expression::structure::Expression;
use expression::s_expression::structure::SExpression;

use parsing::utilities::numerics::representable_numeric;
use parsing::utilities::string::representable_string;
use parsing::utilities::symbols::representable_symbol;

extern crate decimal;
use decimal::d128;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SimplexAtom {
    SimplexSymbol(String),
    SimplexString(String),
    SimplexNumeric(Numeric),
}

impl BaseExpression for SimplexAtom {
    fn get_head(&self) -> Option<SimplexAtom> {
        match self {
            &SimplexAtom::SimplexSymbol(_) => Some(self.clone()),

            &SimplexAtom::SimplexString(_) => {
                Some(SimplexAtom::SimplexSymbol(String::from("String")))
            },

            &SimplexAtom::SimplexNumeric(_) => {
                Some(SimplexAtom::SimplexSymbol(String::from("Number")))
            }
        }
    }

    fn get_rest(&self) -> Option<SimplexAtom> {
        None
    }

    fn to_string(&self) -> String {
        match self {
            &SimplexAtom::SimplexNumeric(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexString(ref n) => n.clone(),
            &SimplexAtom::SimplexSymbol(ref n) => n.clone(),
        }
    }
}
