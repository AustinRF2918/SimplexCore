use std::borrow::Cow;

use atom::numbers::number::Numeric;
use atom::symbols::symbol::Symbol;
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
    SimplexNumeric(Numeric),
    SimplexString(SString),
    SimplexSymbol(Symbol),
}

impl SimplexAtom {
    pub fn new(s: &str) -> SimplexAtom {
        if representable_numeric(s) {
            let n = Numeric::from(s);
            SimplexAtom::SimplexNumeric(n)
        } else if representable_string(s) {
            SimplexAtom::SimplexString(SString::from_str(s).unwrap())
        } else if representable_symbol(s) {
            SimplexAtom::SimplexSymbol(Symbol::from_str(s).unwrap())
        } else {
            panic!("Some invalid input was passed into BaseExpression, maybe develop none case?");
        }
    }
}

impl BaseExpression for SimplexAtom {
    fn to_string(&self) -> String {
        match self {
            &SimplexAtom::SimplexNumeric(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexString(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexSymbol(ref n) => n.to_string().clone(),
        }
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }

    fn get_head(&self) -> &SimplexAtom {
        &self
    }
}
