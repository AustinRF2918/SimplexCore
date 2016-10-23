use atom::numbers::number::Numeric;
use atom::symbols::symbol::Symbol;
use atom::strings::string::SString;

use expression::structures::attributes::BaseExpression;

extern crate decimal;
use decimal::d128;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SimplexAtom {
    SimplexNumeric(Numeric),
    SimplexString(SString),
    SimplexSymbol(Symbol),
}

impl SimplexAtom {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            &SimplexAtom::SimplexNumeric(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexString(ref n) => n.to_string().clone(),
            &SimplexAtom::SimplexSymbol(ref n) => n.to_string().clone(),
        }
    }
}

impl BaseExpression<SimplexAtom> for SimplexAtom {
    fn get_expression_type(&self) -> &str {
        "Simplex`Atom"
    }

    fn get_head_name(&self) -> &str {
        match self {
            &SimplexAtom::SimplexNumeric(num) => {
                match num.simplify() {
                    Numeric::LittleInteger(_) => "Integer",
                    Numeric::LittleReal(_) => "Real",
                    Numeric::NaN => "Symbol",
                }
            }

            &SimplexAtom::SimplexString(_) => "String",
            &SimplexAtom::SimplexSymbol(_) => "Symbol",
        }
    }

    fn get_head(&self) -> &SimplexAtom {
        &self
    }

    fn reduce_expression(self) -> SimplexAtom {
        self
    }

    fn get_int_value(&self) -> Option<i64> {
        match self {
            &SimplexAtom::SimplexNumeric(numeric) => {
                match numeric.simplify() {
                    Numeric::LittleInteger(i) => Some(i),
                    _ => None,
                }
            }

            &SimplexAtom::SimplexString(_) => None,
            &SimplexAtom::SimplexSymbol(_) => None, 
        }
    }

    fn get_float_value(&self) -> Option<d128> {
        match self {
            &SimplexAtom::SimplexNumeric(numeric) => {
                match numeric {
                    Numeric::LittleReal(i) => Some(i),
                    _ => None,
                }
            }

            &SimplexAtom::SimplexString(_) => None,
            &SimplexAtom::SimplexSymbol(_) => None, 
        }
    }

    fn get_string_value(&self) -> Option<&String> {
        match self {
            &SimplexAtom::SimplexNumeric(_) => None,
            &SimplexAtom::SimplexString(ref s) => Some(&s.contents),
            &SimplexAtom::SimplexSymbol(_) => None, 
        }
    }
}
