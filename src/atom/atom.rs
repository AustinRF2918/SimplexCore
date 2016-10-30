use std::borrow::Cow;

use atom::numbers::number::Numeric;
use atom::strings::string::SString;
use atom::numbers::traits;

use expression::traits::BaseExpression;

use parsing::utilities::numerics::representable_numeric;
use parsing::utilities::string::representable_string;
use parsing::utilities::symbols::representable_symbol;

extern crate decimal;
use decimal::d128;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SimplexAtom {
    SimplexSymbol(String),
    SimplexNumeric(Numeric),
    SimplexString(SString),
}

impl BaseExpression for SimplexAtom {
    fn to_string(&self) -> String {
        match self {
            &SimplexAtom::SimplexNumeric(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexString(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexSymbol(ref n) => n.clone(),
        }
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }

    fn get_head(&self) -> SimplexAtom {
        match self {
            &SimplexAtom::SimplexSymbol(_) => self.clone(),

            &SimplexAtom::SimplexString(_) => {
                SimplexAtom::SimplexSymbol("String".to_string())
            },

            &SimplexAtom::SimplexNumeric(_) => {
                SimplexAtom::SimplexSymbol("Number".to_string())
            }
        }
    }
}
