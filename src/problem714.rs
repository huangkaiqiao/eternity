
use log::info;
use std::cmp;

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let width = prices.len();
        let mut memo:Vec<Vec<i32>> = vec![vec![0; width as usize]; width as usize];
        for i in 0..(width-1) {
            for j in (i+1)..width {
                let mut max_value:i32 = prices[j] - prices[i] - 2;
                for k in 0..(i+1) {
                    let lvalue = if k > 0 { memo[0][k] } else { 0 };
                    let rvalue = if k < i { memo[k][i] } else { 0 };
                    max_value = cmp::max(max_value, lvalue + rvalue);
                }
                memo[i][j] = max_value;
                info!("m[{}][{}]:{} ", i, j, memo[i][j])
            }
            info!("\n")
        }
        memo[width-2][width-1]
    }
}