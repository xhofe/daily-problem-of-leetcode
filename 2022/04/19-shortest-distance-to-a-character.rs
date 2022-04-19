// https://leetcode-cn.com/problems/shortest-distance-to-a-character/

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut res = vec![i32::MAX; chars.len()];
        let mut index = -1;
        for i in 0..chars.len() {
            if chars[i] == c {
                index = i as i32;
            }
            if index != -1 {
                res[i] = i as i32 - index;
            }
        }
        index = -1;
        for i in (0..chars.len()).rev() {
            if chars[i] == c {
                index = i as i32;
            }
            if index != -1 {
                res[i] = res[i].min(index - i as i32);
            }
        }
        res
    }
}
