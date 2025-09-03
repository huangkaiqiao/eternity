use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let mut cnt = 0;
        let ln = points.len();
        points.sort_by(|a:&Vec<i32>, b:&Vec<i32>| -> Ordering {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        for i in 1..ln {
            let mut min = std::i32::MAX;
            for j in (0..i).rev() {
                if points[j][1] < points[i][1] {
                    continue;
                }
                if points[j][1] < min {
                    min = points[j][1];
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
