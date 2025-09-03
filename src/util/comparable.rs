use std::

trait Comparable {
    fn compare(&self, other: &Self) -> i32;
}

impl Comparable for i32 {
    fn compare(&self, other: &i32) -> i32 {
        if self < other {
            -1
        } else if self > other {
            1
        } else {
            0
        }
    }
}

impl Comparable for Vec<T> where T: Comparable {
    fn compare(&self, other: &Vec<T>) -> i32 {
        let len1 = self.len();
        let len2 = other.len();
        let min_len = if len1 < len2 { len1 } else { len2 };
        for i in 0..min_len {
            let cmp = self[i].compare(&other[i]);
            if cmp != 0 {
                return cmp;
            }
        }
        len1.compare(&len2)
    }
}