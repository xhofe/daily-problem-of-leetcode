// https://leetcode-cn.com/problems/self-dividing-numbers/

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];
        'outer: for i in left..=right {
            let mut cur = i;
            while cur > 0 {
                let a = cur % 10;
                if a == 0 || i % a != 0 {
                    continue 'outer;
                }
                cur = cur / 10;
            }
            res.push(i);
        }
        res
    }
}