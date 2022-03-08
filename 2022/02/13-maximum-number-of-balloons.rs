// https://leetcode-cn.com/problems/maximum-number-of-balloons/

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt: [i32;5] = [0, 0, 0, 0, 0];
        for ch in text.chars() {
            match ch {
                'b' => { cnt[0] += 1; }
                'a' => { cnt[1] += 1; }
                'l' => { cnt[2] += 1; }
                'o' => { cnt[3] += 1; }
                'n' => { cnt[4] += 1; }
                _ => continue
            }
        }
        cnt[2] /= 2;
        cnt[3] /= 2;
        let mut max_balloons = cnt[0];
        for i in 1..5 {
            if max_balloons > cnt[i] {
                max_balloons = cnt[i];
            }
        }
        max_balloons
    }
}