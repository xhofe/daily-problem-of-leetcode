impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        if final_sum & 1 == 1 {
            return vec![];
        }
        let mut ans = vec![];
        let mut i = 2;
        while i <= final_sum {
            ans.push(i);
            final_sum -= i;
            i += 2;
        }
        if final_sum > 0 {
            *ans.last_mut().unwrap() += final_sum;
        }
        ans
    }
}
