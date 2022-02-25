use std::cmp::max;

use crate::Solution;

impl Solution {
  pub fn compare_version(version1: String, version2: String) -> i32 {
    let vec1: Vec<&str> = version1.as_str().split('.').collect();
    let vec2: Vec<&str> = version2.as_str().split('.').collect();
    let mut num_vec1: Vec<i32> = Vec::new();
    let mut num_vec2: Vec<i32> = Vec::new();
    for s in vec1.iter(){
      let n = s.to_string().parse::<i32>().unwrap();
      num_vec1.push(n);
    }
    for s in vec2.iter(){
      let n = s.to_string().parse::<i32>().unwrap();
      num_vec2.push(n);
    }
    let len = max(vec1.len(), vec2.len());
    let mut i = 0;
    let result = loop {
      if &num_vec1[i] > &num_vec2[i] {
        break 1;
      } else if &num_vec1[i] < &num_vec2[i] {
        break -1;
      } else if i + 1 == len {
        break 0;
      } else {
        num_vec1.push(0);
        num_vec2.push(0);
        i += 1;
      }
    };
    return result;
  }
}