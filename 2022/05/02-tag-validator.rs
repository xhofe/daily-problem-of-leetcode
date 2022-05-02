// https://leetcode-cn.com/problems/tag-validator/

use std::collections::LinkedList;
impl Solution {
    pub fn is_valid(mut code: String) -> bool {
        let mut stack = LinkedList::new();
        let mut code = code.as_bytes();
        while !code.is_empty() {
            if code[0] != b'<' {
                if stack.is_empty() {
                    return false;
                }
                code = &code[1..];
                continue;
            }
            if code.len() == 1 {
                return false;
            }
            if code[1] == b'/' {
                let j = code.iter().position(|&x| x == b'>');
                if j.is_none() { return false; }
                if stack.is_empty() || *stack.back().unwrap() != String::from_utf8_lossy(&code[2..j.unwrap()]) {
                    return false;
                }
                stack.pop_back();
                code = &code[j.unwrap() + 1..];
                if stack.is_empty() && !code.is_empty() { return false; }
            } else if code[1] == b'!' {
                if stack.is_empty() || code.len() < 9 || String::from_utf8_lossy(&code[2..9]) != "[CDATA[".to_owned() {
                    return false;
                }
                let j = String::from_utf8_lossy(code).find("]]>");
                if j.is_none() { return false; }
                code = &code[j.unwrap() + 1..];
            } else {
                let j = code.iter().position(|&x| x == b'>');
                if j.is_none() { return false; }
                let tag = String::from_utf8_lossy(&code[1..j.unwrap()]).to_string();
                if tag.is_empty() || tag.len() > 9 { return false; }
                for char in tag.chars() {
                    if !char.is_uppercase() {
                        return false;
                    }
                }
                stack.push_back(tag);
                code = &code[j.unwrap() + 1..];
            }
        }
        stack.is_empty()
    }
}