impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let x = ((n * (n + 1) / 2) as f64).sqrt() as i32;
        if x * x == (n * (n + 1) / 2) { x } else { -1 }
    }
}
