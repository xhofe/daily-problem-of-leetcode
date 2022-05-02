// https://leetcode-cn.com/problems/smallest-range-i/

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let &max = nums.iter().max().unwrap();
        let &min = nums.iter().min().unwrap();
        if max - min > 2 * k {
            max - min - 2 * k
        } else { 0 }
    }
}