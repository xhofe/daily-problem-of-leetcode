// https://leetcode.cn/problems/set-intersection-size-at-least-two/

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });
        let mut ans = 0;
        let (mut i, mut j) = (-1, -1);
        for interval in intervals {
            let (start, end) = (interval[0], interval[1]);
            if j < start {
                ans += 2;
                i = end - 1;
                j = end;
            }else if i < start {
                ans += 1;
                i = j;
                j = end;
            }
        }
        ans
    }
}
