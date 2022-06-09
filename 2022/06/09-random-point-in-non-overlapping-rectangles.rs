// https://leetcode.cn/problems/random-point-in-non-overlapping-rectangles/

use rand::prelude::*;

struct Solution {
    rects: Vec<(i32, i32, i32, i32)>,
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut sum = 0;
        let mut prefix_sum = vec![0; rects.len()];
        for (i, x) in rects.iter().enumerate() {
            sum += (x[2] - x[0] + 1) * (x[3] - x[1] + 1);
            prefix_sum[i] = sum;
        }
        Self {
            rects: rects
                .into_iter()
                .map(|x| (x[0], x[1], x[2], x[3]))
                .collect(),
            prefix_sum: prefix_sum,
        }
    }

    fn bisect_choose(&self, rand_num: i32) -> usize {
        let mut left = 0;
        let mut right = self.prefix_sum.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if self.prefix_sum[mid] < rand_num {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn pick(&self) -> Vec<i32> {
        let mut rand_num = rand::thread_rng().gen_range(0, self.prefix_sum.last().unwrap()+1);
        let idx = self.bisect_choose(rand_num);
        let (x1, y1, x2, y2) = self.rects[idx];
        let x = rand::thread_rng().gen_range(x1, x2 + 1);
        let y = rand::thread_rng().gen_range(y1, y2 + 1);
        vec![x, y]
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
