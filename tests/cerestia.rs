// use crate::cerestia;
use eternity::{Solution, setup_logger};
use log::info;

#[test]
fn test_count_bits() {
    // setup_logger().unwrap();
    // let sol = Solution{};
    assert_eq!(vec![0,1,1], Solution::count_bits(2));
    assert_eq!(vec![0,1,1,2,1,2], Solution::count_bits(5));
    assert_eq!(vec![0,1,1,2,1,2,2,3,1,2,2], Solution::count_bits(10));
    assert_eq!(
        vec![0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4,1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5,1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5,2,3,3,4,3,4,4,5,3,4,4,5,4,5,5,6,1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5,2,3,3,4,3,4,4,5,3,4,4,5,4,5,5,6,2,3,3,4,3],
        Solution::count_bits(100)
    );
}

#[test]
fn test_fib() {
    assert_eq!(0, Solution::fib(0));
    assert_eq!(1, Solution::fib(2));
    assert_eq!(2, Solution::fib(3));
    assert_eq!(3, Solution::fib(4));
    assert_eq!(13, Solution::fib(7));
    assert_eq!(6765, Solution::fib(20));
}

#[test]
fn test_divisor_game() {
    assert_eq!(false, Solution::divisor_game(1));
    assert_eq!(true, Solution::divisor_game(2));
    assert_eq!(false, Solution::divisor_game(3));
    assert_eq!(true, Solution::divisor_game(10));
    assert_eq!(true, Solution::divisor_game(50));
    assert_eq!(true, Solution::divisor_game(100));
    assert_eq!(true, Solution::divisor_game(500));
    assert_eq!(true, Solution::divisor_game(1000));
}

#[test]
fn test_pascal_triangle() {
    setup_logger().unwrap();
    assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]], Solution::generate(5));
    assert_eq!(vec![vec![1]], Solution::generate(1));
    assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1],vec![1,5,10,10,5,1],vec![1,6,15,20,15,6,1],vec![1,7,21,35,35,21,7,1]], Solution::generate(8));
    assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1],vec![1,5,10,10,5,1],vec![1,6,15,20,15,6,1],vec![1,7,21,35,35,21,7,1],vec![1,8,28,56,70,56,28,8,1],vec![1,9,36,84,126,126,84,36,9,1]], Solution::generate(10))
}