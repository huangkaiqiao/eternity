use log::info;
use crate::Solution;

impl Solution{
    pub fn generate(num_rows:i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut prev = Vec::new();
        if num_rows > 0 {
            res.push(vec![1]);
        }
        if num_rows > 1 {
            res.push(vec![1,1]);
            prev = vec![1,1];
        }
        if num_rows > 2 {
            for i in 3..(num_rows+1) {
                let mut row = Vec::new();
                row.push(1);
                for j in 0..(i-2) {
                    // info!("{}", j);
                    row.push(prev[j as usize]+prev[(j+1) as usize]);
                }
                row.push(1);
                prev = row.clone();
                res.push(row);           
            }
        }
        return res;
    }
}