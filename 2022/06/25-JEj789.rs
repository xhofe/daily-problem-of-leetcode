// https://leetcode.cn/problems/JEj789/

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut last = costs[0].clone();
        for i in 1..costs.len() {
            let mut cur = vec![i32::MAX;3];
            cur[0] = costs[i][0] + last[1].min(last[2]);
            cur[1] = costs[i][1] + last[0].min(last[2]);
            cur[2] = costs[i][2] + last[1].min(last[0]);
            last = cur;
        }
        last.into_iter().min().unwrap()
    }
}
