// https://leetcode-cn.com/problems/lexicographical-numbers/

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        fn dfs(cur: i32, n: i32, list: &mut Vec<i32>) {
            if cur > n { return; }
            list.push(cur);
            let cur = cur * 10;
            for i in 0..10 {
                dfs(cur + i, n, list);
            }
        }
        let mut res = Vec::new();
        for i in 1..10 {
            dfs(i, n, &mut res);
        }
        res
    }
}