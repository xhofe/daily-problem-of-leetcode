// https://leetcode.cn/problems/find-k-th-smallest-pair-distance/

impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut right = nums[n - 1] - nums[0];
        while left <= right {
            let mid = left + ((right - left) >> 1);
            let mut count = 0;
            let mut i = 0;
            for j in 0..n {
                while nums[j]-nums[i] > mid {
                    i+=1;
                }
                count += j - i;
            }
            if count >= k as usize {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
