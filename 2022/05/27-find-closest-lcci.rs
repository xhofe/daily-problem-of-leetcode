// https://leetcode.cn/problems/find-closest-lcci/

impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut ans = i32::MAX;
        let (mut i1, mut i2) = (i32::MAX>>1, i32::MAX);
        words.into_iter().enumerate().for_each(|(i,x)|{
            if x == word1 {
                i1 = i as i32;
            }
            if x == word2 {
                i2 = i as i32;
            }
            ans = ans.min((i1-i2).abs());
        });
        ans
    }
}
