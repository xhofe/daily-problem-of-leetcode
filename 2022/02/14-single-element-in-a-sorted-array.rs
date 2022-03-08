// https://leetcode-cn.com/problems/single-element-in-a-sorted-array/submissions/

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for num in nums {
            res ^= num;
        }
        res
    }
}