// https://leetcode.cn/problems/minimum-cost-to-move-chips-to-the-same-position/

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let even_num = position.iter().filter(|&x| x & 1 == 0).count();
        even_num.min(position.len()-even_num) as i32
    }
}
