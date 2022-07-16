// https://leetcode.cn/problems/cells-with-odd-values-in-a-matrix/

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];
        for ele in indices {
            rows[ele[0] as usize] += 1;
            cols[ele[1] as usize] += 1;
        }
        rows.into_iter()
            .map(|x| cols.iter().filter(|&&y| (x + y) & 1 == 1).count() as i32)
            .sum()
    }
}
