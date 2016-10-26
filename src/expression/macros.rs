#![macro_use]

#[macro_export]
macro_rules! SExpressionFrom {
    ($type_from:ty, $type_to:ty, $expression:ty) => (
    impl SExpressionFrom<$type_from> for $type_to {
        fn push_leave(&mut self, leave: $type_from) {
            self.leaves.push($expression(leave));
        }
    })
}
