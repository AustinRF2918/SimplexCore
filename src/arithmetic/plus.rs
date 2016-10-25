use atom::atom::SimplexAtom;

use atom::symbols::symbol::Symbol;
use atom::numbers::number::Numeric;
use atom::strings::string::SString;

use expression::traits::BaseExpression;
use expression::traits::SExpression;
use expression::traits::SExpressionFrom;
use expression::traits::SExpressionTo;

use expression::structure::Expression;

use arithmetic::subtract::Subtract;

extern crate decimal;
use decimal::d128;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Plus {
    head: SimplexAtom,
    leaves: Vec<Expression>,
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

                &Expression::Sub(ref a) => {
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

impl SExpressionFrom<Subtract> for Plus {
    fn push_leave(&mut self, leave: Subtract) {
        self.leaves.push(Expression::Sub(leave));
    }
}

impl SExpressionTo<SimplexAtom> for Plus {
    fn eval(&self) -> Option<SimplexAtom> {
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
                    match a.eval() {
                        Some(evaluation) => {
                            match evaluation {
                                SimplexAtom::SimplexNumeric(n) => {n_accumulator = n_accumulator + n;},
                                _ => r.push_leave(evaluation),
                            }
                        }
                        None => {
                            r.push_leave(a.clone());
                        }
                    }
                }

                &Expression::Sub(ref a) => {
                    match a.eval() {
                        Some(evaluation) => {
                            match evaluation {
                                SimplexAtom::SimplexNumeric(n) => {n_accumulator = n_accumulator + n;},
                                _ => r.push_leave(evaluation),
                            }
                        }
                        None => {
                            
                            r.push_leave(a.clone());
                        }
                    }
                }
            }
        }

        r.push_leave(SimplexAtom::SimplexNumeric(Numeric::from(n_accumulator)));

        if r.leaves.len() == 1 {
            match r.leaves.pop() {
                Some(Expression::Atomic(SimplexAtom::SimplexNumeric(n))) => Some(SimplexAtom::SimplexNumeric(n)),
                Some(Expression::Atomic(SimplexAtom::SimplexString(st))) => Some(SimplexAtom::SimplexString(st)),
                Some(Expression::Atomic(SimplexAtom::SimplexSymbol(sy))) => Some(SimplexAtom::SimplexSymbol(sy)),
                Some(Expression::Add(a)) => a.eval(),
                Some(Expression::Sub(s)) => s.eval(),
                None => None
            }
        } else {
            None
        }
    }
}
