// https://leetcode-cn.com/problems/simplified-fractions/

fn measure(mut x: i32, mut y: i32) -> i32 {
    let mut z = y;
    while x % y != 0
    {
        z = x % y;
        x = y;
        y = z;
    }
    return z;
}

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        for i in 2..n+1 {
            for j in 1..i {
                if measure(i,j) == 1 {
                    res.push(format!("{}/{}",j,i))
                }
            }
        }
        res
    }
}