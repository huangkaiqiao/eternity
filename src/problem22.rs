use crate::Solution;
use log::info;

impl Solution{
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_owned()];
        }
        let parenthesis = Solution::generate_parenthesis(n-1);
        let mut res = <Vec<String>>::new();
        for p in parenthesis {
            let plen = p.len();
            for i in 0..(plen+1) {
                let tmp = p[0..i].to_owned() + "()" + &p[i..plen];
                if !res.contains(&tmp) {
                    res.push(tmp);
                } 
            }
        }
        return res;
    }
}