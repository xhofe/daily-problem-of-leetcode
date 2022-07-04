// https://leetcode.cn/problems/minimum-absolute-difference/

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        let mut ans = vec![];
        let mut min = i32::MAX;
        for i in 0..arr.len() - 1 {
            let diff = arr[i + 1] - arr[i];
            if diff < min {
                min = diff;
                ans.clear();
            }
            if diff == min {
                ans.push(vec![arr[i], arr[i + 1]]);
            }
        }
        ans
    }
}
