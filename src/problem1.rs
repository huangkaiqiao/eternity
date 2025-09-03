use crate::Solution;
use std::collections::HashMap;

impl Solution{
    // pub fn two_sum(nums: Vec<i32>, target:i32) -> Vec<i32> {
    //     let ln = nums.len();
    //     for i in 0..ln {
    //         for j in 0..ln {
    //             if i != j && nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }
    //     vec![]
    // }

    pub fn two_sum(nums: Vec<i32>, target:i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut ans:Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                Some(value) => {
                    if i != *value {
                        ans.push(i as i32);
                        ans.push(*value as i32);
                    }
                }
                None => {},
            }
            if ans.len() > 0 {
                return ans;
            }
            map.insert(nums[i], i);
        }
        ans
    }
}
