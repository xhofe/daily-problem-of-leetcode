impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0x3f3f3f3f; 1 << cost[0].len()]; cost.len() + 1];
        dp[0][0] = 0;
        for i in 1..=cost.len() {
            for s in 0..1 << cost[0].len() {
                for k in 0..cost[0].len() {
                    if s & (1 << k) == 0 {
                        continue;
                    }
                    dp[i][s] = dp[i][s]
                        .min(dp[i][s ^ (1 << k)] + cost[i - 1][k])
                        .min(dp[i - 1][s] + cost[i - 1][k])
                        .min(dp[i - 1][s ^ (1 << k)] + cost[i - 1][k])
                }
            }
        }
        dp[cost.len()][(1 << cost[0].len()) - 1]
    }
}
