// https://leetcode-cn.com/problems/minimum-height-trees/

use std::collections::VecDeque;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n==1 {
            return vec![0];
        }
        let mut graph = vec![vec![];n as usize];
        edges.iter().for_each(|x|{
            let (a,b) = (x[0] as usize,x[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
        });
        let mut parents = vec![0;n as usize];
        let mut bfs = |start:usize|->usize{
            let mut vis = vec![false;n as usize];
            vis[start] = true;
            let mut q = VecDeque::new();
            q.push_back(start);
            let mut x= start;
            while !q.is_empty() {
                x = q.pop_front().unwrap();
                graph[x].iter().for_each(|&y|{
                    if !vis[y] {
                        vis[y] = true;
                        parents[y] = x as i32;
                        q.push_back(y);
                    }
                })
            }
            x
        };
        let x = bfs(0);
        let mut y = bfs(x) as i32;
        let mut path = vec![];
        parents[x] = -1;
        while y!=-1 {
            path.push(y);
            y = parents[y as usize];
        }
        let m = path.len();
        if m%2==0 {
            return vec![path[m/2-1] as i32,path[m/2] as i32];
        }
        vec![path[m/2] as i32]
    }
}

// 最长的路径中间的节点
// 任意节点最远节点x x最远节点y x->y即最长路径