impl Solution {
    pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut vis = vec![vec![false; land[0].len()]; land.len()];
        fn dfs(land: &Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, x: usize, y: usize) -> i32 {
            let mut ans = 1;
            vis[x][y] = true;
            for &(dx, dy) in [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ]
            .iter()
            {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0
                    && nx < land.len() as i32
                    && ny >= 0
                    && ny < land[0].len() as i32
                    && land[nx as usize][ny as usize] == 0
                    && !vis[nx as usize][ny as usize]
                {
                    ans += dfs(land, vis, nx as usize, ny as usize);
                }
            }
            ans
        }
        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 0 && !vis[i][j] {
                    ans.push(dfs(&land, &mut vis, i, j));
                }
            }
        }
        ans.sort_unstable();
        ans
    }
}
