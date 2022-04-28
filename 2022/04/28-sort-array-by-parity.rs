// https://leetcode-cn.com/problems/sort-array-by-parity/

use std::cmp::Ordering;
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|&x, &y| {
            if (x & 1) == 0 {
                Ordering::Less
            }else {
                Ordering::Greater
            }
        });
        nums
    }
}