// https://leetcode.cn/problems/minimum-moves-to-equal-array-elements-ii/

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let medium = if nums.len() % 2 == 0 {
            (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) / 2
        } else {
            nums[nums.len() / 2]
        };
        let mut count = 0;
        for i in 0..nums.len() {
            count += (nums[i] - medium).abs();
        }
        count
    }
}