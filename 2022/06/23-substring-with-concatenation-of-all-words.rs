// https://leetcode.cn/problems/substring-with-concatenation-of-all-words/

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = vec![];
        let mut counts = std::collections::HashMap::new();
        let num_words = words.len();
        let word_len = words[0].len();
        for w in words {
            *counts.entry(w).or_insert(0) += 1;
        }
        for i in 0..word_len {
            let ss = &s[i..];
            let mut l = 0;
            let mut r = 0;
            let mut counts_clone = counts.clone();
            let mut num_words_clone = num_words;
            while (r as i32) < (ss.len() as i32) - (word_len as i32) + 1 {
                let word = &ss[r..r + word_len];
                if !counts_clone.contains_key(word) {
                    r += word_len;
                    l = r;
                    counts_clone = counts.clone();
                    num_words_clone = num_words;
                    continue;
                }
                let count = counts_clone.entry(word.to_owned()).or_insert(0);
                if *count == 0 {
                    let mut left_word = &ss[l..l + word_len];
                    while left_word != word {
                        *counts_clone.entry(left_word.to_owned()).or_insert(0) += 1;
                        num_words_clone += 1;
                        l += word_len;
                        left_word = &ss[l..l + word_len];
                    }
                    l += word_len;
                } else {
                    *count -= 1;
                    num_words_clone -= 1;
                }
                r += word_len;
                if num_words_clone == 0 {
                    res.push((i + l) as i32);
                }
            }
        }
        res
    }
}
