// https://leetcode-cn.com/problems/mini-parser/

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
use crate::NestedInteger::{Int, List};
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if s.is_empty() {
            return List(vec![]);
        }
        if !s.starts_with("[") {
            return Int(s.parse().unwrap());
        }
        if s.len() <= 2 { return List(vec![]); }
        let mut res = (vec![]);
        let (mut start, mut cnt) = (1, 0);
        let bytes = s.as_bytes();
        for i in 1..s.len() {
            if cnt == 0 && (bytes[i] == b',' || i == s.len() - 1) {
                res.push(Self::deserialize((&s[start..i]).to_owned()));
                start=i+1;
            } else if bytes[i] == b'[' { cnt += 1 } else if bytes[i] == b']' { cnt -= 1 }
        }
        List(res)
    }
}