// https://leetcode-cn.com/problems/convert-1d-array-into-2d-array/

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() != (m * n) as usize { return vec![]; }
        original.chunks_exact(n as usize).map(|arr| arr.to_vec()).collect()
    }
}