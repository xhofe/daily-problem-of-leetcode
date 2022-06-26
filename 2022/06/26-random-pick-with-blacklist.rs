// https://leetcode.cn/problems/random-pick-with-blacklist/

use rand::prelude::*;
struct Solution {
    map: std::collections::BTreeMap<i32, i32>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let mut map = std::collections::BTreeMap::new();
        let mut blacklist = blacklist;
        blacklist.sort();
        if blacklist.is_empty() {
            return Self {
                map: map,
                n: n - blacklist.len() as i32,
            };
        }
        let mut count = 0;
        let mut prev = -1;
        for i in 0..blacklist.len() {
            count += blacklist[i] - prev - 1;
            prev = blacklist[i];
            map.insert(count, blacklist[i]);
        }
        Self {
            map: map,
            n: n - blacklist.len() as i32,
        }
    }

    fn pick(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0, self.n);
        let c = self.map.range(..=r).last();
        if let Some((&k, &v)) = c {
            if r < k {
                r
            }else{
                v + 1 + r - k
            }
        } else {
            r
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */
