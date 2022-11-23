use std::sync::Once;
use eternity::{Solution, setup_logger};
// use eternity::Solution;
// use log::info;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        setup_logger().unwrap();
    })
}

#[test]
// #[ignore]
fn test_is_match() {
    initialize();
    assert_eq!(false, Solution::is_match("aa".to_owned(), "a".to_owned()));
    assert_eq!(true, Solution::is_match("aa".to_owned(), "*".to_owned()));
    assert_eq!(false, Solution::is_match("cb".to_owned(), "?a".to_owned()));
    assert_eq!(true, Solution::is_match("C-77.HongRyeon@outlook.com".to_owned(), "*@*.com".to_owned()));
    assert_eq!(true, Solution::is_match("adceb".to_owned(), "*a*b".to_owned()));
    assert_eq!(true, Solution::is_match("".to_owned(), "******".to_owned()));
    assert_eq!(true, Solution::is_match("".to_owned(), "".to_owned()));
    assert_eq!(false, Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()));
    assert_eq!(true, Solution::is_match("abcabczzzde".to_owned(), "*abc???de*".to_owned()));
    assert_eq!(false, Solution::is_match("mississippi".to_owned(), "m??*ss*?i*pi".to_owned()));
    assert_eq!(false, Solution::is_match("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba".to_owned(), "a*******b".to_owned()));
    assert_eq!(false, Solution::is_match("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_owned(), "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb".to_owned()));
    // assert_eq!(0, Solution::trap(vec![0]));
}