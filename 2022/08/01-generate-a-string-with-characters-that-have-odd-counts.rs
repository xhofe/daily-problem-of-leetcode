impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let is_odd = n & 1 == 1;
        "a".repeat(n as usize - if is_odd { 0 } else { 1 })
            + "b".repeat(if is_odd { 0 } else { 1 }).as_str()
    }
}
