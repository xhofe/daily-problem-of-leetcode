// https://leetcode-cn.com/problems/erect-the-fence/

use std::collections::HashSet;

impl Solution {
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn orientation(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
            (q[1] - p[1]) * (r[0] - p[0]) - (q[0] - p[0]) * (r[1] - p[1])
        }
        let mut res = vec![];
        trees.sort();
        for i in 0..trees.len() {
            while res.len() >= 2 && orientation(&res[res.len() - 2], res.last().unwrap(), &trees[i]) > 0 {
                res.pop();
            }
            res.push(trees[i].clone());
        }
        res.pop();
        for i in (0..trees.len()).rev() {
            while res.len() >= 2 && orientation(&res[res.len() - 2], res.last().unwrap(), &trees[i]) > 0 {
                res.pop();
            }
            res.push(trees[i].clone());
        }
        res.into_iter().collect::<HashSet<_>>().into_iter().collect()
    }
}