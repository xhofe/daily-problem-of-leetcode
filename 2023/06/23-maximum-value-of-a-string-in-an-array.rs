impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter()
            .map(|s| {
                let mut cur = 0;
                for c in s.chars() {
                    if c.is_digit(10) {
                        cur = cur * 10 + c.to_digit(10).unwrap() as i32;
                    } else {
                        return s.len() as i32;
                    }
                }
                cur
            })
            .max()
            .unwrap_or(0)
    }
}
