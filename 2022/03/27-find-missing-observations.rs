// https://leetcode-cn.com/problems/find-missing-observations/

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut sum = mean * (n + rolls.len() as i32) - rolls.iter().sum::<i32>();
        if sum < n {
            return vec![];
        }
        let mean = sum / n;
        let remainder = sum % n;
        if mean > 6 || (mean == 6 && remainder != 0) {
            return vec![];
        }
        let mut res = vec![mean; n as usize];
        for i in 0..remainder {
            res[i as usize] += 1;
        }
        res
    }
}