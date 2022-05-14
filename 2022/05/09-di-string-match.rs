// https://leetcode.cn/problems/di-string-match/

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let (mut l, mut r) = (0, s.len() as i32);
        let mut res = Vec::new();
        s.bytes().enumerate().for_each(|(i, x)| {
            if x == b'I' {
                res.push(l);
                l += 1;
            } else {
                res.push(r);
                r -= 1;
            }
        });
        res.push(l);
        res
    }
}