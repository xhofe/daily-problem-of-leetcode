// https://leetcode.cn/problems/one-away-lcci/

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        let diff = first.len() as i32 - second.len() as i32;
        if diff.abs() > 1 { return false; }
        let bytes1 = first.as_bytes();
        let bytes2 = second.as_bytes();
        let mut i = 0;
        while i < bytes1.len().min(bytes2.len()) {
            if bytes1[i] != bytes2[i] {
                return match diff {
                    -1 => &bytes1[i..] == &bytes2[i + 1..],
                    0 => &bytes1[i + 1..] == &bytes2[i + 1..],
                    _ => &bytes1[i + 1..] == &bytes2[i..]
                };
            }
            i+=1;
        }
        true
    }
}