impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 3]; nums.len() + 1];
        dp[0] = vec![0, i32::MIN, i32::MIN];
        for i in 0..nums.len() {
            let k = nums[i] % 3;
            for j in 0..3 {
                dp[i + 1][j] = dp[i][j].max(dp[i][(3 + j - k as usize) % 3] + nums[i]);
            }
        }
        dp[nums.len()][0]
    }
}
