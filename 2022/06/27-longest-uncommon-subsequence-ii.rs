// https://leetcode.cn/problems/longest-uncommon-subsequence-ii/

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        if strs.len() == 1 {
            return strs[0].len() as i32;
        }
        fn is_subsequence(str: &[u8], sub: &[u8]) -> bool {
            let mut i = 0;
            let mut j = 0;
            while i < str.len() && j < sub.len() {
                if str[i] == sub[j] {
                    j += 1;
                }
                i += 1;
            }
            j == sub.len()
        }
        let mut ans = -1;
        for i in 0..strs.len() {
            let mut is = true;
            for j in 0..strs.len() {
                if i != j && is_subsequence(strs[j].as_bytes(), strs[i].as_bytes()) {
                    is = false;
                    break;
                }
            }
            if is {
                ans = ans.max(strs[i].len() as i32);
            }
        }
        ans
    }
}
