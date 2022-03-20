// https://leetcode-cn.com/problems/the-time-when-the-network-becomes-idle/

use std::collections::VecDeque;
impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let num_point = patience.len();
        let mut graph = vec![vec![]; num_point];
        edges.iter().for_each(|x| {
            graph[x[0] as usize].push(x[1] as usize);
            graph[x[1] as usize].push(x[0] as usize);
        });
        let mut distance = vec![num_point as i32; num_point];
        let mut visited = vec![false; num_point];
        visited[0] = true;
        distance[0] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            graph[cur].iter().for_each(|&x| {
                if !visited[x] {
                    distance[x] = distance[cur] + 1;
                    visited[x] = true;
                    queue.push_back(x);
                }
            })
        }
        let mut ans = 0;
        for i in 1..num_point {
            ans = ans.max(2 * distance[i] + 1 + if distance[i] * 2 <= patience[i] {
                0
            } else {
                // 计算最后一次消息是什么时候发的
                (2 * distance[i] - 1) / patience[i] * patience[i]
            })
        }
        ans
    }
}