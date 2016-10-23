use atom::numbers::number::Numeric;
use atom::symbols::symbol::Symbol;
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
        match (representable_numeric(s), representable_string(s), representable_symbol(s)) {
            (true, _, _) => SimplexAtom::SimplexNumeric(Numeric::from(s)),

            (_, true, _) => {
                match SString::from_str(s) {
                    Some(s) => SimplexAtom::SimplexString(s),

                    None => {
                        panic!(r#"An internal error in the SimplexCore library occured: representable_string(s)
                        in the parsing library returned true, ensuring that our numeric is parseable, however
                        SString::from_str(s) returned None."#);
                    }
                }
            }

            (_, _, true) => {
                match Symbol::from_str(s) {
                    Some(s) => SimplexAtom::SimplexSymbol(s),

                    None => {
                        panic!(r#"An internal error in the SimplexCore library occured: representable_symbol(s)
                        in the parsing library returned true, ensuring that our numeric is parseable, however
                        Symbol::from_str(s) returned None."#);
                    }
                }
            }

            _ => SimplexAtom::SimplexSymbol(Symbol::from_str("Simplex`UnknownParse").unwrap()),
        }
    }
}
