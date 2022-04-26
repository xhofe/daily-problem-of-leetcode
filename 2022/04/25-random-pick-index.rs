// https://leetcode-cn.com/problems/random-pick-index/

use std::collections::HashMap;

struct Solution {
    map: HashMap<i32, Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        nums.into_iter().enumerate().for_each(|(i, x)| {
            let list = map.entry(x).or_insert(vec![]);
            list.push(i as i32);
        });
        Self { map }
    }

    fn pick(&self, target: i32) -> i32 {
        let list = self.map.get(&target).unwrap();
        let rand_number = rand::random::<usize>() % list.len();
        list[rand_number]
    }
}