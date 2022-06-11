// https://leetcode.cn/problems/count-different-palindromic-subsequences/

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.into_bytes();
        let MOD = 1e9 as i32 + 7;
        let n = s.len();
        let mut next_right = vec![vec![0; 4]; n + 2];
        let mut next_left = vec![vec![0; 4]; n + 2];
        for i in 1..=n {
            for j in 0..4 {
                next_left[i][j] = if (s[i - 1] - b'a') == j as u8 {
                    i
                } else {
                    next_left[i - 1][j]
                };
                next_right[n - i + 1][j] = if (s[n - i] - b'a') == j as u8 {
                    n - i + 1
                } else {
                    next_right[n - i + 2][j]
                };
            }
        }
        let mut dp = vec![vec![0; n + 2]; n + 2];
        for i in 0..=n {
            dp[i][i] = 1;
        }
        for ln in 2..=n {
            for i in 1..=n + 1 - ln {
                let j = i + ln - 1;
                if s[i - 1] != s[j - 1] {
                    dp[i][j] = ((dp[i + 1][j] + dp[i][j - 1]) % MOD - dp[i + 1][j - 1] + MOD) % MOD;
                } else {
                    let l = next_right[i + 1][(s[i - 1] - b'a') as usize];
                    let r = next_left[j - 1][(s[i - 1] - b'a') as usize];
                    if l < j && r > i {
                        if l == r {
                            dp[i][j] = (2 * dp[i + 1][j - 1] % MOD + 1) % MOD;
                        } else {
                            dp[i][j] = (2 * dp[i + 1][j - 1] % MOD - dp[l + 1][r - 1] + MOD) % MOD;
                        }
                    } else {
                        dp[i][j] = (2 * dp[i + 1][j - 1] % MOD + 2) % MOD;
                    }
                }
            }
        }
        dp[1][n]
    }
}
