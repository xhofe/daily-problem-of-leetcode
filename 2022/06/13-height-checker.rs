// https://leetcode.cn/problems/height-checker/

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort();
        heights
            .into_iter()
            .enumerate()
            .filter(|&(i, x)| x != expected[i])
            .count() as i32
    }
}
