// https://leetcode.cn/problems/frog-position-after-t-seconds/

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut g = vec![vec![]; n as usize + 1];
        for edge in edges {
            g[edge[0] as usize].push(edge[1] as usize);
            g[edge[1] as usize].push(edge[0] as usize);
        }
        let mut vis = vec![false; n as usize + 1];
        vis[1] = true;
        fn dfs(
            edges: &Vec<Vec<usize>>,
            pos: usize,
            probability: f64,
            vis: &mut Vec<bool>,
            t: i32,
            target: i32,
        ) -> f64 {
            if t == 0 {
                if pos as i32 == target {
                    return probability;
                } else {
                    return 0.0;
                }
            }
            let cnt = edges[pos]
                .iter()
                .filter(|&x| !vis[*x])
                .count();
            if cnt == 0 {
                if pos as i32 == target {
                    return probability;
                } else {
                    return 0.0;
                }
            }
            let mut ans = 0.0;
            for &next in edges[pos].iter() {
                if !vis[next] {
                    vis[next] = true;
                    ans += dfs(edges, next, probability / cnt as f64, vis, t - 1, target);
                    vis[next] = false;
                }
            }
            ans
        }
        dfs(&g, 1, 1.0, &mut vis, t, target)
    }
}
