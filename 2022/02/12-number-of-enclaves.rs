// https://leetcode-cn.com/problems/number-of-enclaves/

impl Solution {
    pub fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if i >= grid.len() || i < 0 || j >= grid[0].len() || j < 0 {
            return;
        }
        if grid[i][j] == 0 { return; }
        grid[i][j] = 0;
        Solution::dfs(grid,i-1,j);
        Solution::dfs(grid,i+1,j);
        Solution::dfs(grid,i,j-1);
        Solution::dfs(grid,i,j+1);
    }
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        // let mut grid = grid.clone();
        let (mut i, mut j) = (0, 0);
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (i==0||j==0||i==grid.len()-1||j==grid[0].len()-1)&&grid[i][j]==1 {
                    Solution::dfs(&mut grid, i, j);
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}
