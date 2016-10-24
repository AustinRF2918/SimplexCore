use atom::atom::SimplexAtom;

use expression::structures::attributes::BaseExpression;
use expression::structures::attributes::SExpression;
use expression::structures::attributes::SExpressionFrom;
use expression::structures::attributes::BuiltinExpression;
// use expression::structures::attributes::MetaExpression;

use atom::numbers::number::Numeric;
use atom::symbols::symbol::Symbol;

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
    leaves: Vec<Expression>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression {
    Add(Plus),
    Atomic(SimplexAtom)
}

impl Plus {
    #[allow(dead_code)]
    pub fn new() -> Plus {
        Plus {
            head: SimplexAtom::SimplexSymbol(Symbol::from_str("Plus").unwrap()),
            leaves: Vec::new(),
        }
    }
}
impl BaseExpression for Plus {
    fn get_expression_type(&self) -> &str {
        "Simplex`MExpression"
    }

    fn get_head_name(&self) -> &str {
        match self.head {
            SimplexAtom::SimplexSymbol(ref s) => s.to_string().as_str(),
            _ => "Simplex`Invalid",
        }
    }

    fn get_head(&self) -> &SimplexAtom {
        &self.head
    }

    fn reduce_expression(&self) -> Option<SimplexAtom> {
        let mut x = self.eval();
        let mut y = Plus::new();

        for i in x.leaves {
            match i {
                Expression::Add(ref x) => {
                    match x.reduce_expression() {
                        Some(x) => {
                            y.push_leave(x)
                        }
                        None => {
                        }
                    }
                }
                Expression::Atomic(ref x) => {y.push_leave(x.clone())}
            }
        }

        let mut z = y.eval();

        if z.leaves.len() == 1 {
            match y.leaves[0] {
                Expression::Atomic(ref t) => {
                    match t {
                        &SimplexAtom::SimplexNumeric(_) => Some(t.clone()),
                        _ => None
                    }
                }
                _ => None
            }
        } else {
            None
        }
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

    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.get_head_name());
        s.push('[');
        for leave in &self.leaves {
            match leave {
                &Expression::Atomic(SimplexAtom::SimplexNumeric(ref n)) => {
                    s.push_str(n.to_string().as_str());
                },

                &Expression::Atomic(SimplexAtom::SimplexString(ref st)) => {
                    s.push_str(st.to_string().as_str());
                },

                &Expression::Atomic(SimplexAtom::SimplexSymbol(ref sy)) => {
                    s.push_str(sy.to_string().as_str());
                },

                &Expression::Add(ref a) => {
                    s.push_str(a.to_string().as_str());
                }
            }
            s.push(',');
            s.push(' ');
        }
        s.pop();
        s.pop();
        s.push(']');

        s
    }

    

}

impl SExpression for Plus {
    fn get_leaves(&self) -> &Vec<Expression> {
        &self.leaves
    }
}

impl SExpressionFrom<SimplexAtom> for Plus {
    fn push_leave(&mut self, leave: SimplexAtom) {
        self.leaves.push(Expression::Atomic(leave));
    }
}

impl SExpressionFrom<Plus> for Plus {
    fn push_leave(&mut self, leave: Plus) {
        self.leaves.push(Expression::Add(leave));
    }
}

impl BuiltinExpression<SimplexAtom> for Plus {
    fn eval(&self) -> Plus {
        let mut r = Plus::new();
        let mut n_accumulator = Numeric::from(0);

        for atomic in &self.leaves {
            match atomic {
                &Expression::Atomic(SimplexAtom::SimplexNumeric(n)) => {
                    n_accumulator = n_accumulator + n;
                },

                &Expression::Atomic(ref a) => {
                    r.push_leave(a.clone());
                },

                &Expression::Add(ref a) => {
                    r.push_leave(a.eval());
                }
            }
        }

        r.push_leave(SimplexAtom::SimplexNumeric(Numeric::from(n_accumulator)));
        r
    }
}
