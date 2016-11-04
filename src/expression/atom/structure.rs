use expression::atom::numbers::number::Numeric;

use expression::traits::BaseExpression;
use expression::structure::SimplexPointer;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SimplexAtom {
    SimplexSymbol(String),
    SimplexString(String),
    SimplexNumeric(Numeric),
}

impl Drop for SimplexAtom {
    fn drop(&mut self) {
        println!("[Heavyweight] Dropping Atom: {} which is copyable.", self.to_string());
    }
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

    fn replace_symbol(&mut self, symbol: &BaseExpression, new: &BaseExpression) -> SimplexPointer {
        if self.to_string() == symbol.to_string() {
            *self = SimplexAtom::from(new.to_string())
        } 

        SimplexPointer::from(self.clone())
    }

    fn uniq_id(&self) -> String {
        "Copyable".to_string()
    }

    fn set_uniq_id(&mut self, id: u64) {
        panic!("This is a copyable primitive, it doesn't have a memory location visible because it is ALWAYS copied.");
    }
}
