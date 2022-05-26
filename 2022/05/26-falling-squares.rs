// https://leetcode.cn/problems/falling-squares/

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = positions.len();
        let mut heights = vec![0; n];
        for (i,p) in positions.iter().enumerate() {
            let (l1,r1) = (p[0],p[0]+p[1]-1);
            heights[i] = p[1];
            for j in 0..i {
                let q = &positions[j];
                let (l2,r2) = (q[0],q[0]+q[1]-1);
                if r1 >= l2 && r2 >= l1 {
                    heights[i] = heights[i].max(heights[j]+p[1]);
                }
            }
        }
        for i in 1..n {
            heights[i] = heights[i].max(heights[i-1]);
        }
        heights
    }
}
