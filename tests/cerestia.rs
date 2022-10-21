use std::sync::Once;
use eternity::{Solution, setup_logger};
use log::info;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        setup_logger().unwrap();
    })
}

#[test]
#[ignore]
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
#[ignore]
fn test_fib() {
    assert_eq!(0, Solution::fib(0));
    assert_eq!(1, Solution::fib(2));
    assert_eq!(2, Solution::fib(3));
    assert_eq!(3, Solution::fib(4));
    assert_eq!(13, Solution::fib(7));
    assert_eq!(6765, Solution::fib(20));
}

#[test]
#[ignore]
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
#[ignore]
fn test_pascal_triangle() {
    assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]], Solution::generate(5));
    assert_eq!(vec![vec![1]], Solution::generate(1));
    assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1],vec![1,5,10,10,5,1],vec![1,6,15,20,15,6,1],vec![1,7,21,35,35,21,7,1]], Solution::generate(8));
    assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1],vec![1,5,10,10,5,1],vec![1,6,15,20,15,6,1],vec![1,7,21,35,35,21,7,1],vec![1,8,28,56,70,56,28,8,1],vec![1,9,36,84,126,126,84,36,9,1]], Solution::generate(10))
}

#[test]
#[ignore]
fn test_longest_palindrome() {
    initialize();
    info!("test_longest_palindrome");
    assert_eq!("bab", Solution::longest_palindrome("babad".to_owned()));
    assert_eq!("bb", Solution::longest_palindrome("cbbd".to_owned()));
    assert_eq!("a", Solution::longest_palindrome("a".to_owned()));
    assert_eq!("bbbbbbbbbbb", Solution::longest_palindrome("bbbbbbbbbbb".to_owned()));
    assert_eq!("aaaa", Solution::longest_palindrome("aaaa".to_owned()));
    assert_eq!("aaaaa", Solution::longest_palindrome("aaaaa".to_owned()));
    assert_eq!("a", Solution::longest_palindrome("ac".to_owned()));
    assert_eq!("redivider", Solution::longest_palindrome("sqqrdnntqmqyacredividerlkyy".to_owned()));
}

#[test]
#[ignore]
fn test_is_match() {
    initialize();
    assert_eq!(false, Solution::is_match("aa".to_owned(), "a".to_owned()));
    assert_eq!(true, Solution::is_match("aa".to_owned(), "a*".to_owned()));
    assert_eq!(false, Solution::is_match("ab".to_owned(), "a".to_owned()));
    assert_eq!(false, Solution::is_match("aab".to_owned(), "a".to_owned()));
    assert_eq!(false, Solution::is_match("cabbbb".to_owned(), "c.a*ab*".to_owned()));
    assert_eq!(true, Solution::is_match("caabbbb".to_owned(), "c.a*ab*".to_owned()));
    assert_eq!(true, Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
    assert_eq!(false, Solution::is_match("a".to_owned(), "ab*a".to_owned()));
}

#[test]
#[ignore]
fn test_generate_parenthesis() {
    initialize();
    let mut gp3 = Solution::generate_parenthesis(3);
    gp3.sort();
    assert_eq!(vec!["((()))", "(()())", "(())()", "()(())", "()()()"], gp3);
    let mut gp4 = Solution::generate_parenthesis(4);
    gp4.sort();
    assert_eq!(vec!["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"], gp4);
    assert_eq!(vec!["()"], Solution::generate_parenthesis(1));
} 

#[test]
#[ignore]
fn test_longest_valid_parenttheses() {
    initialize();
    assert_eq!(2, Solution::longest_valid_parentheses("(()".to_owned()));
    assert_eq!(4, Solution::longest_valid_parentheses(")()())".to_owned()));
    assert_eq!(0, Solution::longest_valid_parentheses("".to_owned()));
    assert_eq!(8, Solution::longest_valid_parentheses("()()()()".to_owned()));
    assert_eq!(8, Solution::longest_valid_parentheses("()(())()".to_owned()));
    assert_eq!(62, Solution::longest_valid_parentheses("((((())())()))(()((()()(())())((((((()(())(())((())(((((())))())))(((())())(())))))()(()())())((()((".to_owned()));
}

#[test]
// #[ignore]
fn test_trap() {
    initialize();
    assert_eq!(6, Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
    assert_eq!(9, Solution::trap(vec![4,2,0,3,2,5]));
    assert_eq!(4786199, Solution::trap(vec![29102,35081,90363,20759,88881,24893,16770,24826,5168,29825,13868,98264,28017,91846,41740,54111,26071,76528,32975,15549,74578,8360,20845,98625,94370,77630,67437,50415,26887,10625,96447,70286,12156,76772,57816,95099,90448,55333,65975,3790,8938,69120,79539,42779,49757,27543,3351,57039,18889,51750,76243,26864,24584,58471,74072,53154,32980,88990,8327,29736,12049,9871,39100,31362,97586,49803,75804,66343,43383,42529,2105,32630,91658,81482,98509,21784,5049,12234,47566,10033,30035,80889,26904,49630,12966,91299,41604,10902,62663,75983,42096,65086,34930,22927,98000,40935,32685,19775,79281,29276]));
    assert_eq!(0, Solution::trap(vec![0]));
}
