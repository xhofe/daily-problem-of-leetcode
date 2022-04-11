// https://leetcode-cn.com/problems/count-numbers-with-unique-digits/

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        for i in 1..=n as usize {
            let (mut sum, mut j) = (9 as i32, i);
            while j > 1 {
                j -= 1;
                sum *= 10 - j as i32;
            }
            dp[i] = sum + dp[i - 1];
        }
        dp[n as usize]
    }
}