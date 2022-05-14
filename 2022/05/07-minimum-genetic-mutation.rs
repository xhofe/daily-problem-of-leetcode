// https://leetcode.cn/problems/minimum-genetic-mutation/

use std::collections::LinkedList;
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        fn can(a: &[u8], b: &[u8]) -> bool {
            let mut diff = 0;
            for i in 0..a.len() {
                if a[i] != b[i] {
                    diff += 1;
                    if diff > 1 { return false; }
                }
            }
            diff == 1
        }
        if !bank.contains(&end) { return -1; }
        let mut queue = LinkedList::new();
        queue.push_back(&start);
        let mut vis = vec![false; bank.len()];
        let mut ans = 0;
        while !queue.is_empty() {
            ans += 1;
            let size = queue.len();
            for _ in 0..size {
                let cur = queue.pop_front().unwrap();
                for i in 0..bank.len() {
                    if vis[i] { continue; }
                    if can(cur.as_bytes(), bank[i].as_bytes()) {
                        if bank[i] == end {
                            return ans;
                        }
                        queue.push_back(&bank[i]);
                        vis[i] = true;
                    }
                }
            }
        }
        -1
    }
}