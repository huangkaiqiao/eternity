use crate::Solution;

impl Solution {

    // pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    //     let mut original:Vec<i32> = vec![0];
    //     for item in derived {
    //         if item == 0 {
    //             original.push(*original.last().unwrap());
    //         } else if item == 1 {
    //             original.push(1 - original.last().unwrap());
    //         }
    //     }
    //     original.first() == original.last()
    // }

    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut count = 0;
        for item in derived {
            if item == 1 {
                count += 1
            }
        }
        count % 2 == 0
    }
}