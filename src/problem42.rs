use crate::Solution;

impl Solution{

    // DP
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


    // Brute Force
    // pub fn trap(height: Vec<i32>) -> i32 {
    //     let n = height.len();
    //     let mut res:i32 = 0;
    //     let mut i = 0;
    //     while i < n - 1 {
    //         let mut right_max = 0;
    //         let mut next = 0;
    //         let mut j = i + 1;
    //         while j < n {
    //             if height[j] >= height[i] {
    //                 right_max = height[j];
    //                 next = j;
    //                 break;
    //             }
    //             if height[j] >= right_max {
    //                 right_max = height[j];
    //                 next = j;
    //             }
    //             j += 1;
    //         }
    //         for k in i+1..(next+1) {
    //             let depth = height[i] - height[k];
    //             if depth > 0 {
    //                 res += depth;
    //                 // info!("i:{} j:{} res:{}", i, k, res)
    //             }
    //         }
    //         let dis = height[i] - right_max;
    //         if dis > 0 {
    //             res -= dis * (next-i) as i32;
    //             // info!("i:{} j:{} res:{}", i, j, res)
    //         }
    //         i = next;
    //     }
    //     return res;
    // }

    // https://leetcode.com/problems/trapping-rain-water/solution/
    // Approach 1:Brute force
    // Approach 2:Dynamic Programming
    // Approach 3:Using stacks
    // Approach 4:Using 2 pointers
}