// https://leetcode.cn/problems/largest-triangle-area/

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut ans: f64 = 0.0;
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let x1 = points[j][0]-points[i][0];
                let y1 = points[j][1]-points[i][1];
                for k in j + 1..points.len() {
                    let x2 = points[k][0]-points[i][0];
                    let y2 = points[k][1]-points[i][1];
                    ans = ans.max((x1*y2-x2*y1).abs() as f64)
                }
            }
        }
        ans/2.0
    }
}