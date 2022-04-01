// https://leetcode-cn.com/problems/array-of-doubled-pairs/

use std::collections::{HashMap};

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_by(|a,b|b.abs().cmp(&a.abs()));
        let mut map = HashMap::new();
        for x in arr {
            if map.contains_key(&(2*x)) {
                let count = map.get(&(2*x)).unwrap();
                if *count==1 {
                    map.remove(&(2*x));
                }else {
                    map.insert(2*x,count-1);
                }
            }else {
                let count = map.entry(x).or_insert(0);
                *count+=1;
            }
        }
        map.is_empty()
    }
}