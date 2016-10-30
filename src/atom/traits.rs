use atom::numbers::number::Numeric;
use atom::strings::string::SString;

use atom::atom::SimplexAtom;

use parsing::utilities::numerics::representable_numeric;
use parsing::utilities::string::representable_string;
use parsing::utilities::symbols::representable_symbol;

numeric_int_explicit_conversion!(i8, SimplexAtom::SimplexNumeric, SimplexAtom);
numeric_int_explicit_conversion!(i16, SimplexAtom::SimplexNumeric, SimplexAtom);
numeric_int_explicit_conversion!(i32, SimplexAtom::SimplexNumeric, SimplexAtom);
numeric_int_explicit_conversion!(i64, SimplexAtom::SimplexNumeric, SimplexAtom);
numeric_int_explicit_conversion!(u8, SimplexAtom::SimplexNumeric, SimplexAtom);
numeric_int_explicit_conversion!(u16, SimplexAtom::SimplexNumeric, SimplexAtom);
numeric_int_explicit_conversion!(u32, SimplexAtom::SimplexNumeric, SimplexAtom);

numeric_float_explicit_conversion!(f32, SimplexAtom::SimplexNumeric, SimplexAtom);
numeric_float_explicit_conversion!(f64, SimplexAtom::SimplexNumeric, SimplexAtom);

impl<'a> From<&'a str> for SimplexAtom {
    fn from(s: &str) -> SimplexAtom {
        if representable_numeric(s) {
            let n = Numeric::from(s);
            SimplexAtom::SimplexNumeric(n)
        } else if representable_string(s) {
            SimplexAtom::SimplexString(SString::from_str(s).unwrap())
        } else if representable_symbol(s) {
            SimplexAtom::SimplexSymbol(s.to_string())
        } else {
            // TODO: USE DYNAMIC ERROR TYPE HERE.
            panic!("Some invalid input was passed into BaseExpression, maybe develop none case?");
        }
    }
}
