use std::cmp;
use std::collections::HashMap;
// use log::{debug, error, info, trace, warn};
use log::info;

struct Solution;

impl Solution {
    pub fn _minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut sum:i32 = 0;
        for c in s1.chars() {
            sum += c as i32;
        }
        for c in s2.chars() {
            sum += c as i32;
        }
        sum
    }

    pub fn minimum_delete_sum1(s1: String, s2: String) -> i32 {
        info!("s1:{}\ts2:{}", s1, s2);
        let mut sum:i32 = 0;
        let mut tmp:i32 = 0;
        if s1.len() == 1 && s2.len() == 1 {
            if s1.char_at(0) == s2.char_at(0) {
                return 0;
            }
            sum = s1.char_at(0) as i32 + s2.char_at(0) as i32;
            // info!("case1: {}", sum);
            return sum
        }
        if s1.len() == 0 || s2.len() == 0 {
            for c in s1.chars() {
                sum += c as i32;
            }
            for c in s2.chars() {
                sum += c as i32;
            }
            // info!("case2: {}", sum);
            return sum
        }
        // let s2tmp = s2.clone();
        // let s1tmp = s1.clone();
        let s2_1 = s2.clone();
        let s1_1 = s1.clone();
        let s2_2 = s2.clone();
        let s1_2 = s1.clone();
        let _s2 = s2.clone();
        let _s1 = s1.clone();
        let sum1 = s1.char_at(0) as i32 + Solution::minimum_delete_sum(s1.get(1..).unwrap().to_string(), s2);
        let sum2 = s2_1.char_at(0) as i32 + Solution::minimum_delete_sum(s1_1, s2_1.get(1..).unwrap().to_string());
        sum = cmp::min(sum1, sum2);
        if s1_2.char_at(0) == s2_2.char_at(0) {
            let sum3 = Solution::minimum_delete_sum(s1_2.get(1..).unwrap().to_string(), s2_2.get(1..).unwrap().to_string());
            // info!("sum3:{}", sum3);
            sum = cmp::min(sum, sum3);
        }
        // info!("s1:{}\ts2:{}\tsum1:{}\tsum2:{}", _s1, _s2, sum1, sum2);
        // info!("case3: {}", sum);
        sum
    }

    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let rows:usize = s1.len();
        let columns:usize = s2.len();
        // info!("minimum_delete_sum(rows:{}, columns:{})", rows, columns);
        let mut memo:Vec<Vec<i32>> = vec![vec![0; columns+1 as usize]; rows+1 as usize];
        for i in (0..rows+1).rev() {
            for j in (0..columns+1).rev() {
                // info!("{}", &233.to_string());
                let s1i = if i < rows { s1.as_bytes()[i] as i32 } else { 0 };
                let s2j = if j < columns { s2.as_bytes()[j] as i32 } else { 0 };
                if i == rows {
                    let tmp = if j < columns { memo[i][j+1] } else { 0 };
                    memo[i][j] = tmp + s2j;
                    // info!("case1 memo[{}][{}]:{}", i, j, memo[i][j]);
                    continue;
                }
                if j == columns {
                    let tmp = if i < rows { memo[i+1][j] } else { 0 };
                    memo[i][j] = tmp + s1i;
                    // info!("case2 memo[{}][{}]:{}", i, j, memo[i][j]);
                    continue;
                }
                let mut sum = memo[i+1][j+1] + s1i + s2j;
                if s1i == s2j {
                    sum = memo[i+1][j+1];
                }
                let tmp1 = memo[i][j+1] + s2j;
                let tmp2 = memo[i+1][j] + s1i;
                sum = i32::min(sum, tmp1);
                sum = i32::min(sum, tmp2);
                memo[i][j] = sum;
                // info!("sum: {}, tmp1: {}, tmp2: {}", sum, tmp1, tmp2);
                // info!("case3 memo[{}][{}]:{}", i, j, memo[i][j]);
            }
        }
        memo[0][0]
    }
}

pub trait CharSequence {
    fn char_at(&self, i:usize) -> char;
}

impl CharSequence for String {
    fn char_at(&self, i:usize) -> char {
        self.chars().nth(i).unwrap()
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    // println!("Hello, world!");
    // let s = Solution{};
    let result = setup_logger();
    info!("{:?}", result.unwrap());
    // println!("{}", );
    let r = Solution::minimum_delete_sum(String::from("sea"), String::from("eat"));
    info!("{}", r);
    let r = Solution::minimum_delete_sum(String::from("delete"), String::from("leet"));
    info!("{}", r);
}
