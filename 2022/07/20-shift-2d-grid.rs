// https://leetcode.cn/problems/shift-2d-grid/

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m,n) = (grid.len(),grid[0].len());
        let mut res = vec![vec![0;n];m];
        for i in 0..m {
            for j in 0..n {
                let pos = (i * n + j + k as usize) % (m *n);
                let newI = pos / n;
                let newJ = pos % n;
                res[newI][newJ] = grid[i][j];
            }
        }
        res
    }
}
