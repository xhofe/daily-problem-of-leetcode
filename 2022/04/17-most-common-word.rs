// https://leetcode-cn.com/problems/most-common-word/

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let set = banned.into_iter().collect::<HashSet<_>>();
        let mut map = HashMap::new();
        let paragraph = paragraph.to_lowercase().replace('!'," ")
            .replace('?'," ").replace('\''," ").replace(','," ")
            .replace(';'," ").replace('.'," ");
        let words = paragraph.split_whitespace();
        let mut max = 0;
        let mut ans = "".to_owned();
        for x in words {
            if x.is_empty() || set.contains(x) { continue; }
            let count = map.entry(x).or_insert(0);
            *count += 1;
            if *count > max {
                max = *count;
                ans = x.to_owned();
            }
        }
        ans
    }
}