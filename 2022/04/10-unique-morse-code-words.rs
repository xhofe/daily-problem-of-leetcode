// https://leetcode-cn.com/problems/unique-morse-code-words/

use std::collections::HashSet;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        static TABLE: [&str; 26] = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];
        let mut set = HashSet::new();
        words.iter().for_each(|x|{
            let mut s = String::new();
            for &x in x.as_bytes() {
                s.push_str(TABLE[(x-b'a') as usize]);
            }
            set.insert(s);
        });
        set.len() as i32
    }   
}