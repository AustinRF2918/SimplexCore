/// BaseExpression
///
/// This is the structure that we use to represent a
/// base expression in Simplex: It consists of a pattern
/// sequence, isFormatted, represented by a boolinear which
/// will tell us we have a self referencing expression, and
/// last evaluated, which contains the last evaluation performed
/// in a linear evaluation.
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BaseExpression {
    //options: Option<Vec<Symbols>>,
    pattern_sequence: bool,
    is_formatted: bool,
    //last_evaluated: Option<Evaluation>
}

impl BaseExpression {
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
    #[allow(dead_code)]
    pub fn new() -> BaseExpression {
        BaseExpression {
            //options: None
            pattern_sequence: false,
            is_formatted: true,
            //last_evluated: None
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
    fn iterate_rules(&self, /*rules: Vec<Rule>*/) -> Option<BaseExpression>{
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
    pub fn apply_rules(&self /*rules: Rule, */ /*evaluation: Evaluation*/, level: u8, options: (Option<u8>, Option<u8>)) -> Option<BaseExpression>{
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