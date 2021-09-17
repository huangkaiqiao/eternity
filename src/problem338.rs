use crate::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res:Vec<i32> = Vec::new();
        for i in 0..n+1 {
            let mut c:i32 = 0;
            let mut tmp:i32 = i;
            while tmp > 0 {
                c = c + (tmp&1);
                tmp = tmp / 2;
            }
            res.push(c);
        }
        res    
    }
}