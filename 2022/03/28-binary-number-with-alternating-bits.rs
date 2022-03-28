// https://leetcode-cn.com/problems/binary-number-with-alternating-bits/

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut last = n & 1;
        while n != 0 {
            n >>= 1;
            if n & 1 == last {
                return false;
            }
            last = n & 1;
        }
        return true;
    }
}