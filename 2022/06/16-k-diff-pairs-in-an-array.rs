// https://leetcode.cn/problems/k-diff-pairs-in-an-array/

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut right = 1;
        let mut ans = 0;
        let mut set = std::collections::HashSet::new();
        while right < nums.len() {
            while left < right && nums[right] - nums[left] >= k {
                if nums[right] - nums[left] == k {
                    if !set.contains(&nums[right]) {
                        ans += 1;
                        set.insert(nums[right]);
                    }
                }
                left += 1;
            }
            right += 1;
        }
        ans
    }
}
