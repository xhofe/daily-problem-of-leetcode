// https://leetcode-cn.com/problems/k-th-smallest-in-lexicographical-order/

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let (mut k, mut cur, n) = ((k - 1) as i64, 1 as i64, n as i64);
        while k > 0 {
            let (mut count, mut first, mut last) = (0, cur, cur + 1);
            while first <= n {
                count += last.min(n + 1) - first;
                first *= 10;
                last *= 10;
            }
            if count <= k {
                cur += 1;
                k -= count;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur as i32
    }
}