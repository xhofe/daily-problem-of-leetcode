// https://leetcode.cn/problems/kth-smallest-number-in-multiplication-table/

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (mut left, mut right) = (1, m * n);
        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = mid / n * n;
            for i in mid/n+1..=m {
                count += mid / i;
            }
            if count < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}