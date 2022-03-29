// https://leetcode-cn.com/problems/maximize-the-confusion-of-an-exam/

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        fn max_consecutive_char(answer_key: &[u8], k: i32, c: u8) -> i32 {
            let (mut left, mut right, mut ans, mut sum) = (0, 0, 0, 0);
            while right < answer_key.len() {
                sum += if answer_key[right] == c { 0 } else { 1 };
                while sum > k {
                    sum -= if answer_key[left] == c { 0 } else { 1 };
                    left += 1
                }
                ans = ans.max(right + 1 - left);
                right += 1;
            }
            ans as i32
        }
        return max_consecutive_char(answer_key.as_bytes(), k, b'T').max(max_consecutive_char(answer_key.as_bytes(), k, b'F'));
    }
}

// 滑动窗口：窗口中最多有k个'T'或'F'