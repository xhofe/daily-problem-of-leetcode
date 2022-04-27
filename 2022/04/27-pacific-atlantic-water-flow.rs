// https://leetcode-cn.com/problems/pacific-atlantic-water-flow/

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty() || heights[0].is_empty() { return vec![]; }
        let mut pacific = vec![vec![false; heights[0].len()]; heights.len()];
        let mut atlantic = vec![vec![false; heights[0].len()]; heights.len()];
        fn dfs(heights: &Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, i: usize, j: usize) {
            if vis[i][j] { return; }
            vis[i][j] = true;
            let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for d in dirs {
                let (ni, nj) = (i as i32 + d.0, j as i32 + d.1);
                if 0 <= ni && ni < heights.len() as i32 && 0 <= nj && nj < heights[0].len() as i32 && heights[ni as usize][nj as usize] >= heights[i][j] {
                    dfs(heights, vis, ni as usize, nj as usize);
                }
            }
        }
        for i in 0..heights.len() {
            dfs(&heights, &mut pacific, i, 0);
            dfs(&heights, &mut atlantic, i, heights[0].len() - 1);
        }
        for i in 0..heights[0].len() {
            dfs(&heights, &mut pacific, 0, i);
            dfs(&heights, &mut atlantic, heights.len() - 1, i);
        }
        let mut res = vec![];
        pacific.into_iter().enumerate().for_each(|(i, x)| {
            x.into_iter().enumerate().for_each(|(j, y)| {
                if y && atlantic[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            })
        });
        res
    }
}
