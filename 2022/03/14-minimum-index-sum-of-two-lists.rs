// https://leetcode-cn.com/problems/minimum-index-sum-of-two-lists/

use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = list1.into_iter().enumerate().map(|x| (x.1, x.0)).collect::<HashMap<_, _>>();
        let mut res = vec![];
        let mut min = usize::MAX;
        list2.into_iter().enumerate().for_each(|(i, x)| {
            if let Some(&j) = map.get(&x) {
                if i + j == min {
                    res.push(x);
                } else if i + j < min {
                    res.clear();
                    res.push(x);
                    min = i + j;
                }
            }
        });
        res
    }
}