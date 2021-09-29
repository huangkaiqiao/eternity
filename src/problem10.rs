use crate::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let slen = s.len();
        let plen = p.len();
        let chars: Vec<char> = p.chars().collect::<Vec<_>>();
        for i in 0..plen {
            let mut j = i;
            while chars[j].is_alphabetic() {
                j += 1;
            }
            if 
            
        }
        false
    }
}