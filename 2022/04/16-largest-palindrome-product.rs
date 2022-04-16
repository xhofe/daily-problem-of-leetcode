// https://leetcode-cn.com/problems/largest-palindrome-product/

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 { return 9; }
        let mut upper = 10_i64.pow(n as u32) - 1;
        let mut left = upper;
        loop {
            let mut p = left;
            let mut x = left;
            while x > 0 {
                p = p * 10 + x % 10;
                x /= 10;
            }
            x = upper;
            while x * x >= p {
                if p % x == 0 {
                    return (p % 1337) as i32;
                }
                x -= 1;
            }
            left -= 1;
        }
    }
}