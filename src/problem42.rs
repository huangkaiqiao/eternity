use crate::Solution;

impl Solution{

    pub fn trap(height: Vec<i32>) -> i32 {
        // println!("************************************");
        let n = height.len();
        let mut left = 0;
        let mut right = n -1;
        let mut max_left = 0;
        let mut max_right = 0;
        let mut res = 0;
        while left < right {
            if height[left] < height[right] {
                if max_left < height[left] { max_left = height[left]; }
                else {res += max_left - height[left]; }
                left += 1;
            } else {
                if max_right < height[right] { max_right = height[right]; }
                else { res += max_right - height[right]; }
                right -= 1
            }
            // println!("{} {} {}", left, res, right);
        }
        res
    }
}