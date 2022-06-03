// https://leetcode.cn/problems/consecutive-numbers-sum/

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let n_is_even = n & 1 == 0;
        while i * (i + 1) / 2 <= n {
            i += 1;
            let i_is_even = i & 1 == 0;
            let i_div_2_is_even = i >> 1 & 1 == 0;
            if i_is_even && i_div_2_is_even && !n_is_even {
                // only even
                continue;
            }
            if i_is_even && !i_div_2_is_even && n_is_even {
                // only odd
                continue;
            }
            let mut cur = i;
            if cur & 1 == 0 {
                cur >>= 1;
            }
            if n % cur == 0 {
                let mut div = n / cur;
                let div_is_even = div & 1 == 0;
                if div_is_even && i_is_even {
                    continue;
                }
                if !i_is_even {
                    div <<= 1;
                }
                if div >= (i + 1) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
