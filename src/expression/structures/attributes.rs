// This API must be Rustified: It is way to Pythonic and because
// Rust can automatically implemenet comparables and keys, we
// should do it idiomatically instead... 

// As a result, I have developed no test cases.

#[allow(dead_code)]
pub trait KeyGenerator {
    fn get_sort_key(&self) -> u64;
}

#[allow(dead_code)]
pub trait KeyComparable {
    fn greater_than(&self, other: &Self) -> bool;
    fn greater_equal(&self, other: &Self) -> bool;
    fn less_than(&self, other: &Self) -> bool;
    fn less_equal(&self, other: &Self) -> bool;
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool;
}

impl <T> KeyComparable for T
    where T: KeyGenerator  {
    fn less_than(&self, other: &Self) -> bool {
        self.get_sort_key() < other.get_sort_key()
    }

    fn less_equal(&self, other: &Self) -> bool{
        self.get_sort_key() <= other.get_sort_key()
    }

    fn greater_than(&self, other: &Self) -> bool{
        self.get_sort_key() > other.get_sort_key()
    }

    fn greater_equal(&self, other: &Self) -> bool{
        self.get_sort_key() >= other.get_sort_key()
    }

    fn equal(&self, other: &Self) -> bool {
        self.get_sort_key() == other.get_sort_key()
    }

    fn not_equal(&self, other: &Self) -> bool {
        self.get_sort_key() != other.get_sort_key()
    }
}
