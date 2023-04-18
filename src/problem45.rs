use crate::Solution;

impl Solution {
  pub fn jump(nums: Vec<i32>) -> i32 {
      if nums.len() == 1 {
        return 0;
      }
      let mut stop:Vec<i32> = vec![];
      let mut step:Vec<i32> = vec![];
      for i in 0..nums.len() {
        stop.push(0);
        step.push(0);
      }
      step[1] = 1;
      for i in 2..nums.len() {
        if i as i32 <= stop[i-1] + nums[stop[i-1] as usize] {
          stop[i] = stop[i-1];
          step[i] = step[i-1];
        } else {
          // for j in (0..(i-1)).rev() {
          for j in (0..i).rev() {
            if i - j <= nums[j] as usize {
              stop[i] = j as i32;
              step[i] = step[j] + 1;
            }
          }
        }
      }
      step[nums.len()-1]
  }  
}