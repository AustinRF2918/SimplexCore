use std::fmt;

use expression::atom::numbers::number::Numeric;
use expression::traits::{BaseExpression, CompileableExpression};
use expression::structure::SimplexPointer;

#[derive(Clone, Eq, PartialEq)]
pub enum SimplexAtom {
    SimplexSymbol(String),
    SimplexString(String),
    SimplexNumeric(Numeric),
}

impl fmt::Debug for SimplexAtom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SimplexAtom::SimplexSymbol(ref s) => {
                write!(f, "SimplexSymbol: {}", s)
            }

            &SimplexAtom::SimplexString(ref s) => {
                write!(f, "SimplexString: '{}'", s)
            }

            &SimplexAtom::SimplexNumeric(ref n) => {
                write!(f, "SimplexNumeric: '{}'", n.as_str())
            }
        }
    }
}

impl BaseExpression for SimplexAtom {
    fn get_head(&self) -> Option<SimplexPointer> {
        Some(SimplexPointer::from(self.clone()))
    }

    fn get_rest(&self) -> Option<SimplexPointer> {
        None
    }

    fn to_string(&self) -> String {
        match self {
            &SimplexAtom::SimplexNumeric(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexString(ref n) => n.clone(),
            &SimplexAtom::SimplexSymbol(ref n) => n.clone(),
        }
    }

    fn replace_symbol(&self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        if self.to_string() == symbol.to_string() {
            println!("Swapping atomic {} from {} to {}", self.as_str(), symbol.as_str(), new.as_str());
            SimplexPointer::from(new.to_string())
        }  else {
            println!("Not Swapping atomic {} from {} to {}", self.as_str(), symbol.as_str(), new.as_str());
            SimplexPointer::from(self.clone())
        }
    }

    fn evaluate(&self, v: &Vec<SimplexPointer>) -> SimplexPointer {
        SimplexPointer::from(self.clone())
    }
}
