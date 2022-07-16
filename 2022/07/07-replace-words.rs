// https://leetcode.cn/problems/replace-words/

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut dict = dictionary;
        dict.sort_by(|a, b| a.len().cmp(&b.len()));
        sentence
            .split_whitespace()
            .into_iter()
            .map(|x| {
                for root in &dict {
                    if x.starts_with(root) {
                        return root.to_string();
                    }
                }
                x.to_string()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
