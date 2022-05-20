// https://leetcode.cn/problems/find-right-interval/

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut starts = intervals
            .iter()
            .enumerate()
            .map(|x| (x.1[0], x.0))
            .collect::<Vec<_>>();
        let mut ends = intervals
            .iter()
            .enumerate()
            .map(|x| (x.1[1], x.0))
            .collect::<Vec<_>>();
        starts.sort_by_key(|x| x.0);
        ends.sort_by_key(|x| x.0);
        let mut res = vec![-1; n];
        let mut j = 0;
        for x in ends {
            while j < n && starts[j].0 < x.0 {
                j += 1;
            }
            if j < n {
                res[x.1] = starts[j].1 as i32;
            }
        }
        res
    }
}