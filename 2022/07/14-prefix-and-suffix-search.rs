// https://leetcode.cn/problems/prefix-and-suffix-search/

struct WordFilter {
    words: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        for i in (0..self.words.len()).rev() {
            if self.words[i].starts_with(&pref) && self.words[i].ends_with(&suff) {
                return i as i32;
            }
        }
        -1
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(pref, suff);
 */
