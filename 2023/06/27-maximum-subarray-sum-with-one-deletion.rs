impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let (mut dp0, mut dp1, mut ans) = (arr[0], 0, arr[0]);
        for i in 1..arr.len() {
            dp1 = dp0.max(dp1 + arr[i]);
            dp0 = dp0.max(0) + arr[i];
            ans = ans.max(dp0).max(dp1);
        }
        ans
    }
}
