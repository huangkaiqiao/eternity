
use std::cmp;
use crate::Solution;

// pub struct Solution;

impl Solution {
    // pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    //     let width = prices.len();
    //     let mut memo:Vec<Vec<i32>> = vec![vec![0; width as usize]; width as usize];
    //     for i in 0..(width-1) {
    //         for j in (i+1)..width {
    //             let mut max_value:i32 = prices[j] - prices[i] - 2;
    //             for k in 0..(i+1) {
    //                 let lvalue = if k > 0 { memo[0][k] } else { 0 };
    //                 let rvalue = if k < i { memo[k][i] } else { 0 };
    //                 max_value = cmp::max(max_value, lvalue + rvalue);
    //             }
    //             memo[i][j] = max_value;
    //             info!("m[{}][{}]:{} ", i, j, memo[i][j])
    //         }
    //         info!("\n")
    //     }
    //     memo[width-2][width-1]
    // }

    // pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    //     let len = prices.len();
    //     let mut memo:Vec<Vec<i32>> = vec![vec![0; len+1 as usize]; len+1 as usize];
    //     for offset in 1..len {
    //         // for i in 0..(len-1) {
    //         for i in 0..len {
    //             let j = i + offset;
    //             if j < len {
    //                 for k in (i+1)..j {
    //                     memo[i][j] = cmp::max(memo[i][j], memo[i][k] + memo[k][j]);
    //                 }
    //                 memo[i][j] = cmp::max(memo[i][j], prices[j] - prices[i] - fee);
    //                 // info!("memo[{}][{}]:{}", i, j, memo[i][j]);
    //             }
    //         }
    //     }
    //     memo[0][len-1]
    // }

    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut p0:i64 = 0;
        let mut p1:i64 = i32::MIN as i64;

        for p in prices {
            let p0_old = p0;
            p0 = cmp::max(p0, p1 + (p-fee) as i64);
            p1 = cmp::max(p1, p0_old - p as i64);
        }

        p0 as i32
    }
}