// https://leetcode-cn.com/problems/rotate-function/

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        fn f(arr: &Vec<i32>) -> i32 {
            arr.into_iter().enumerate().map(|(i, x)| {
                i as i32 * x
            }).sum()
        }
        let mut pre = f(&nums);
        let mut ans = pre;
        let sum = nums.iter().sum::<i32>();
        for i in 0..nums.len() {
            pre = sum - nums.len() as i32 * nums[nums.len() - i - 1] + pre;
            ans=ans.max(pre);
        }
        ans
    }
}