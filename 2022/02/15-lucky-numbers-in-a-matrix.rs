// https://leetcode-cn.com/problems/lucky-numbers-in-a-matrix/

impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rows=vec![0;matrix.len()];
        let mut cols = vec![0;matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j]<matrix[i][rows[i]] {
                    rows[i] = j;
                }
                if matrix[i][j]>matrix[cols[j]][j] {
                    cols[j] = i;
                }
            }
        }
        let mut res = Vec::new();
        for i in 0..matrix.len() {
            if i == cols[rows[i]] {
                res.push(matrix[i][rows[i]])
            }
        }
        res
    }
}