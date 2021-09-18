use crate::Solution;

impl Solution{
    // pub fn fib(n: i32) -> i32 {
    //     if n > 2 {
    //         Solution::fib(n-1) + Solution::fib(n-2)
    //     } else if n > 0 {
    //         1
    //     } else {
    //         n
    //     }
    // }
    pub fn fib(n: i32) -> i32 {
        let sqrt5 = 5_f64.sqrt();
        let exp1 = ((1_f64+sqrt5)/2_f64).powf(n as f64);
        let exp2 = ((1_f64-sqrt5)/2_f64).powf(n as f64);
        ((exp1 - exp2)/sqrt5) as i32
    }
}