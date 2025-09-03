use std::sync::Once;
use eternity::{Solution, setup_logger};

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        setup_logger().unwrap();
    })
}

struct TestCase {
    points: Vec<Vec<i32>>,
    expect: i32,
}

/**
 * cargo test --test problem3025
 */
#[test]
fn test_number_of_pairs() {
    initialize();
    let data = [
        (vec![[1,1],[2,2],[3,3]],       0),
        (vec![[6,2],[4,4],[2,6]],       2),
        (vec![[3,1],[1,3],[1,1]],       2),
        (vec![[2,4],[4,2],[6,4],[4,6]], 4),
        (vec![[0,2],[3,1],[6,0],[1,5]], 3)
    ];
    let mut cases: Vec<TestCase> = Vec::new();
    for (arr, expect) in data {
        let points: Vec<Vec<i32>> = arr.into_iter().map(|a| a.to_vec()).collect();
        cases.push(TestCase { points, expect });
    }
    for case in cases {
        let actual = Solution::number_of_pairs(case.points.clone());
        assert_eq!(case.expect, actual, "case {:?}", case.points);
    }
}