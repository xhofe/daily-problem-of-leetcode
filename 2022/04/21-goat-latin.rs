// https://leetcode-cn.com/problems/goat-latin/

use std::collections::{HashSet};
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let set = vec!["a", "e", "i", "o", "u"].into_iter().collect::<HashSet<_>>();
        sentence.split_whitespace().enumerate().map(|(i, x)| -> String {
            let first = &x[0..1];
            let mut tmp = x.to_string();
            if !set.contains(&first.to_lowercase() as &str) {
                tmp = (&x[1..]).to_string() + first;
            }
            tmp + "ma" + &("a".repeat(i + 1))
        }).collect::<Vec<_>>().join(" ")
    }
}
