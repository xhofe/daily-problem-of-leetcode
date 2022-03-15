// https://leetcode-cn.com/problems/count-number-of-maximum-bitwise-or-subsets/

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;
        Solution::backtracking(&nums, 0, 0, &mut max, &mut count);
        count
    }
    // The maximum value can also be calculated in advance to facilitate pruning
    fn backtracking(nums: &Vec<i32>, i: usize, cur: i32, max: &mut i32, count: &mut i32) {
        if i >= nums.len() {
            if cur >= *max {
                if cur > *max {
                    *max = cur;
                    *count = 0;
                }
                *count += 1;
            }
            return;
        }
        Solution::backtracking(nums,i+1,cur,max,count);
        Solution::backtracking(nums,i+1,cur|nums[i],max,count);
    }
}