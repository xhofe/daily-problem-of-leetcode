impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let c = n / k as usize;
        let mut dp = vec![vec![std::i32::MAX >> 1; n]; 1 << n];
        for i in 0..n {
            dp[1 << i][i] = 0;
        }
        for s in 0..1 << n {
            for i in 0..n {
                if s & (1 << i) == 0 {
                    continue;
                }
                for j in 0..n {
                    if s & (1 << j) == 1 {
                        continue;
                    }
                    let t = s | (1 << j);
                    dp[t][j] = dp[t][j].min(if s.count_ones() as usize % c == 0 {
                        dp[s][i]
                    } else if nums[j] > nums[i] {
                        dp[s][i] + nums[j] - nums[i]
                    } else {
                        dp[t][j]
                    })
                }
            }
        }
        *dp[(1 << n) - 1]
            .iter()
            .filter(|&&x| x < std::i32::MAX >> 1)
            .min()
            .unwrap_or(&-1)
    }
}
