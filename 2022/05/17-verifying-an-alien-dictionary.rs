// https://leetcode.cn/problems/verifying-an-alien-dictionary/

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut order_map = [0; 26];
        for (i, c) in order.bytes().enumerate() {
            order_map[(c - b'a') as usize] = i;
        }
        let compare = |a:&String,b:&String|{
            let a = a.as_bytes();
            let b = b.as_bytes();
            for i in 0..a.len().min(b.len()) {
                if a[i] != b[i] {
                    return order_map[(a[i]-b'a') as usize] < order_map[(b[i]-b'a') as usize];
                }
            }
            a.len() <= b.len()
        };
        for i in 1..words.len() {
            if !compare(&words[i-1], &words[i]) {
                return false;
            }
        }
        true
    }
}