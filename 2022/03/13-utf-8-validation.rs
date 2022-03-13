// https://leetcode-cn.com/problems/utf-8-validation/

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut i = 0;
        while i < data.len() {
            if Solution::get_bit(data[i], 8) == 1 {
                let mut j = 7;
                let cur = data[i];
                if Solution::get_bit(cur,7) == 0 {
                    return false;
                }
                while j >= 5 && Solution::get_bit(cur, j) == 1 {
                    i += 1;
                    j -= 1;
                    if i >= data.len() {
                        return false;
                    }
                    if Solution::get_bit(data[i], 8) != 1 || Solution::get_bit(data[i], 7) != 0 {
                        return false;
                    }
                }
                if j == 4 && Solution::get_bit(cur,j) == 1 {
                    return false;
                }
            }
            i += 1;
        }
        true
    }
    fn get_bit(x: i32, n: usize) -> i32 {
        (x >> (n - 1)) & 1
    }
}