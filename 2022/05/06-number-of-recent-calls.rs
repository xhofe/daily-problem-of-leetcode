// https://leetcode.cn/problems/number-of-recent-calls/

use std::collections::LinkedList;

struct RecentCounter {
    pings: LinkedList<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self { pings: LinkedList::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);
        while *self.pings.front().unwrap() < t - 3000 {
            self.pings.pop_front();
        }
        self.pings.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */