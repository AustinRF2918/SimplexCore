use atom::atom::SimplexAtom;
use atom::traits;
use parsing::utilities::string::representable_string;
use parsing::utilities::numerics::representable_numeric;

use expression::structures::integrity::checks::ensure_context;
use expression::structures::attributes::BaseExpression;

use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use atom::numbers::number::Numeric;

use std::borrow::Cow;

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
pub struct CoreExpression {
    //options: Option<Vec<Symbols>>,
    head: SimplexAtom,
    leaves: LinkedList<Rc<CoreExpression>>
    //pattern_sequence: bool,
    // is_formatted: bool,
    //last_evaluated: Option<Evaluation>
}

impl CoreExpression {
    /// Constructs a new BaseExpression.
    ///
    /// Note that this basically sets our expression to be a
    /// totally empty type: It is up to us to actually push
    /// our various rules to this structure.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut BaseExpression = BaseExpression::new()
    /// ```
    pub fn new(head_name: &str) -> CoreExpression {
        CoreExpression {
            //options: None
            head: SimplexAtom::from(head_name),
            leaves: LinkedList::new(),
            //pattern_sequence: false,
            //is_formatted: true,
            //last_evluated: None
        }
    }

    pub fn get_head_name<'a>(&'a self) -> Cow<'a, str>{
        match self.head {
            SimplexAtom::SimplexSymbol(ref s) => {
                Cow::Owned(ensure_context(s.to_string()).clone())
            },
            _ => Cow::Owned(ensure_context(self.head.get_head_name()))
        }
    }

    pub fn push_leave(&mut self, n: Rc<CoreExpression>) {
        self.leaves.push_back(n.clone());
    }

    pub fn eval(&self) -> CoreExpression {
        match self.head {
            SimplexAtom::SimplexSymbol(ref s) => {
                if s.to_string().as_str() == "Plus" {
                    let mut new_expr = CoreExpression::new("Plus");
                    let mut last_evaluated = Numeric::from(0);
                        for item in &self.leaves {
                            match item.head {
                                SimplexAtom::SimplexNumeric(x) => {
                                    last_evaluated = last_evaluated + x
                                }
                                _ => {
                                    new_expr.push_leave(Rc::new(CoreExpression::new(last_evaluated.to_string().as_str())));
                                    last_evaluated = Numeric::from(0);
                                }
                            }
                        }
                    new_expr.push_leave(Rc::new(CoreExpression::new(last_evaluated.to_string().as_str())));
                    new_expr
                }
                else {
                    self.clone()
                }
            },
            SimplexAtom::SimplexNumeric(ref n) => {
                self.clone()
            }
            SimplexAtom::SimplexString(ref s) => {
                self.clone()
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self.head {
            SimplexAtom::SimplexSymbol(ref s) => {
                let mut rep = String::new();
                rep.push_str(s.to_string().as_str());
                rep.push('[');
                for (item_num, item) in self.leaves.iter().enumerate() {
                    if item_num != 0 {
                        rep.push(' ')
                    } 

                    rep.push_str(item.to_string().as_str());
                    if item_num != self.leaves.len() - 1 {
                        rep.push(',');
                    }

                }

                rep.push(']');
                rep
            },
            SimplexAtom::SimplexNumeric(ref n) => {
                n.to_string()
            }
            SimplexAtom::SimplexString(ref s) => {
                s.to_string()
            }
        }
    }

    /// Taking a currently created BaseExpression and creates
    /// a copy, applying rules linearly to our expression and
    /// outputting the modified structure: This is a pure function.
    /// Note that in the case that our rules don't throw a "none"
    /// this works as one would expect it to, however if our
    /// expression can not be evaluated it returns a None and
    /// the system upstream will have to determine how to deal
    /// with the issue
    ///
    /// # Examples
    ///
    /// ```
    /// let mut base_exp = BaseExpression::new();
    /// let rule = SympyEngine("Integrate");
    /// let mut new_exp = base_exp.iterate_rules( vec![rule]);
    /// ```
    /// TODO: True implementation once Rule data
    /// structure is properly implemented.
    #[allow(dead_code)]
    fn iterate_rules(&self, /*rules: Vec<Rule>*/) -> Option<CoreExpression>{
        /*
        let mut new_expression = self.clone(); 
        for rule in rules {
            rule.apply(new_expression);
            if new_expression != None {
                (new_expression, true)
            }
        }
        */

        None
    }

    // TODO: Add more descriptive names to
    // destructured elements (l1, l2): What
    // do these things even mean?

    // TODO: Make this faster by using
    // lazy returns: That way if nothing
    // has changed, we simply denote that
    // via the use of an enumerative type.

    // TODO: True implementation once Rule data
    // structure is properly implemented.

    #[allow(dead_code)]
    pub fn apply_rules(&self /*rules: Rule, */ /*evaluation: Evaluation*/, level: u8, options: (Option<u8>, Option<u8>)) -> Option<CoreExpression>{
        match options {
            (Some(l1), None) => {
                if level < l1 {
                    None
                } else {
                    self.iterate_rules(/*rules*/)
                }
            }, (None, Some(l2)) => {
                if level > l2 {
                    None
                } else {
                    self.iterate_rules(/*rules*/)
                }
            }, (Some(l1), Some(l2)) => {
                if level < l1 {
                    None
                } else if level > l2 {
                    None
                } else {
                    self.iterate_rules(/*rules*/)
                }
            }, _ => {
                self.iterate_rules(/*rules*/)
            }
        }
    }
}
