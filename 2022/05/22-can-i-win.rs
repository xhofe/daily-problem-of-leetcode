// https://leetcode.cn/problems/can-i-win/

use std::collections::HashMap;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if max_choosable_integer >= desired_total {
            return true;
        }
        if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
            return false;
        }
        fn win(status: i32, left: i32, n: i32, cache: &mut HashMap<i32,bool>)-> bool {
            if cache.contains_key(&status) {
                return cache[&status];
            }
            for i in 0..n {
                let cur = 1 << i;
                if (status & cur) == 0 {
                    if left <= i+1 || !win(status | cur, left - i - 1, n, cache) {
                        cache.insert(status, true);
                        return true;
                    }
                }
            }
            cache.insert(status, false);
            false
        }
        let mut cache = HashMap::new();
        win(0, desired_total, max_choosable_integer, &mut cache)
    }
}