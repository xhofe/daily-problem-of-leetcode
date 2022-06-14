// https://leetcode.cn/problems/diagonal-traverse/

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let (m, n) = (mat.len() as i32, mat[0].len() as i32);
        for i in 0..(m + n - 1) {
            if i & 1 == 1 {
                let mut x = 0.max(i - n + 1);
                let mut y = i.min(n - 1);
                while x < m && y >= 0 {
                    ans.push(mat[x as usize][y as usize]);
                    x += 1;
                    y -= 1;
                }
            } else {
                let mut x = i.min(m - 1);
                let mut y = (i - m + 1).max(0);
                while x >= 0 && y < n {
                    ans.push(mat[x as usize][y as usize]);
                    x -= 1;
                    y += 1;
                }
            }
        }
        ans
    }
}
