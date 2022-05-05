// https://leetcode-cn.com/problems/reorder-data-in-log-files/

use std::cmp::Ordering;
impl Solution {
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        logs.sort_by(|x,y|{
            let s1 = x.splitn(2," ").collect::<Vec<_>>();
            let s2 = y.splitn(2," ").collect::<Vec<_>>();
            let is_digit1 = s1[1].chars().nth(0).unwrap().is_digit(10);
            let is_digit2 = s2[1].chars().nth(0).unwrap().is_digit(10);
            if !is_digit1 && !is_digit2 {
                let cmp = s1[1].cmp(s2[1]);
                return match cmp {
                    Ordering::Equal => s1[0].cmp(s2[0]),
                    _ => cmp
                }
            }
            if is_digit1 {
                if is_digit2 {
                    return Ordering::Equal;
                }
                return Ordering::Greater;
            }
            return Ordering::Less;
        });
        logs
    }
}