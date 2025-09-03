use eternity;

use eternity::Solution;

#[test]
fn test_two_sum() {
    let data = [
        (vec![2,7,11,15], 9, vec![0,1]),
        (vec![3,2,4],     6, vec![1,2]),
        (vec![3,3],       6, vec![0,1])
    ];
    for (nums, target, expect) in data {
        let mut actual = Solution::two_sum(nums.clone(), target);
        actual.sort();
        assert_eq!(expect, actual, "case {:?}", nums);
    }
}
