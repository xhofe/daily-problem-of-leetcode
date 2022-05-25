// https://leetcode.cn/problems/unique-substrings-in-wraparound-string/

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut lens = [0; 26];
        let bytes = p.as_bytes();
        let mut count = 1;
        for i in 1..bytes.len() {
            lens[(bytes[i - 1] - b'a') as usize] = lens[(bytes[i - 1] - b'a') as usize].max(count);
            if (bytes[i] == b'a' && bytes[i - 1] == b'z')
                || (bytes[i] > bytes[i - 1] && bytes[i] - bytes[i - 1] == 1)
            {
                count += 1;
            } else {
                count = 1;
            }
        }
        lens[(bytes[bytes.len() - 1] - b'a') as usize] =
            lens[(bytes[bytes.len() - 1] - b'a') as usize].max(count);
        lens.into_iter().sum()
    }
}
