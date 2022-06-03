// https://leetcode.cn/problems/matchsticks-to-square/

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            return false;
        }
        let avg = sum / 4;
        let mut dp = vec![-1; 1 << matchsticks.len()];
        dp[0] = 0;
        for s in 1..dp.len() {
            for (k, &v) in matchsticks.iter().enumerate() {
                if s >> k & 1 == 0 {
                    continue;
                }
                let prev = s & (!(1 << k));
                if dp[prev] >= 0 && dp[prev] + v <= avg {
                    dp[s] = (dp[prev] + v) % avg;
                    break;
                }
            }
        }
        *dp.last().unwrap() == 0
    }
}
