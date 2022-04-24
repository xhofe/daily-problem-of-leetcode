// https://leetcode-cn.com/problems/binary-gap/

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut ans = 0;
        while (n & 1) == 0 {
            n >>= 1;
        }
        if n == 1 { return 0; }
        n >>= 1;
        let mut tmp = 1;
        while n != 0 {
            if (n & 1) == 0 {
                tmp += 1;
            } else {
                ans = ans.max(tmp);
                tmp = 1;
            }
            n >>= 1;
        }
        ans
    }
}