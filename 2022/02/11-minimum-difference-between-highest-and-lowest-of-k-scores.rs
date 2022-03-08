// https://leetcode-cn.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut res = i32::MAX;
        for i in 0..nums.len() - (k - 1) as usize {
            let t = nums.get(i + (k-1) as usize).unwrap() - nums.get(i).unwrap();
            if t < res { res = t }
        }
        res
    }
}