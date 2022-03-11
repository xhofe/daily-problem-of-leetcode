// https://leetcode-cn.com/problems/count-nodes-with-the-highest-score/

impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut graph = vec![(-1, -1); parents.len()];
        for (index, parent) in parents.into_iter().enumerate() {
            if parent == -1 { continue; }
            let children = graph[parent as usize];
            if children.0 == -1 {
                graph[parent as usize].0 = index as i32;
            } else {
                graph[parent as usize].1 = index as i32;
            }
        }
        let mut max:i64 = 0;
        let mut count = 0;
        Solution::dfs(0, &graph, &mut max, &mut count);
        count
    }
    pub fn dfs(node: i32, graph: &Vec<(i32, i32)>, max: &mut i64, count: &mut i32) -> i32 {
        if node == -1 { return 0; }
        let len = graph.len();
        let children = graph[node as usize];
        let left = Solution::dfs(children.0, graph, max, count);
        let right = Solution::dfs(children.1, graph, max, count);
        let parent: i32 = len as i32 - 1 - left - right;
        let mut score: i64 = 1;
        if parent != 0 { score *= parent as i64; }
        if left != 0 { score *= left as i64; }
        if right != 0 { score *= right as i64; }
        if score > *max {
            *max = score;
            *count = 1;
        } else if score == *max {
            *count += 1;
        }
        1 + left + right
    }
}