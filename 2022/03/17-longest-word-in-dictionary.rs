// https://leetcode-cn.com/problems/longest-word-in-dictionary/

use std::collections::HashSet;
impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort();
        let mut set = HashSet::new();
        let mut res = "".to_string();
        for x in words {
            if x.len() == 1 || set.contains(&x[..x.len() - 1]) {
                res = if x.len() > res.len() { x.to_string() } else { res };
                set.insert(x);
            }
        }
        res
    }
}