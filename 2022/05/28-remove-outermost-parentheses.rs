// https://leetcode.cn/problems/remove-outermost-parentheses/

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans = String::new();
        let mut count = 0;
        let mut last = 0;
        for (i, b) in s.bytes().enumerate() {
            count += if b == b'(' { 1 } else { -1 };
            if count == 0 {
                ans.push_str(&s[last + 1..i]);
                last = i + 1;
            }
        }
        ans
    }
}
