// https://leetcode-cn.com/problems/maximum-nesting-depth-of-the-parentheses/

use std::cmp::max;
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut ans = 0;
        let mut cur = 0;
        for c in s.chars() {
            if c == '(' {
                cur += 1;
                ans = max(ans, cur);
            }
            if c == ')' {
                cur -= 1
            }
        }
        ans
    }
}