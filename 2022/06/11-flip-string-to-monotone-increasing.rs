// https://leetcode.cn/problems/flip-string-to-monotone-increasing/

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut cur = s.bytes().filter(|&c| c == b'0').count() as i32;
        let mut ans = cur;
        for c in s.bytes() {
            cur += if c == b'0' { -1 } else { 1 };
            ans = ans.min(cur);
        }
        ans
    }
}
