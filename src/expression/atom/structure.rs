use expression::atom::numbers::number::Numeric;
use expression::traits::{BaseExpression, CompileableExpression};
use expression::structure::SimplexPointer;
// Remove pointers: Only API that should use pointer is pointer itself:
// ESPECIALLY NOT ATOMICS!!!

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
