// https://leetcode.cn/problems/generate-random-point-in-a-circle/

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let radians = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        let radius = rand::random::<f64>().sqrt() * self.radius;
        let x = self.x_center + radius * radians.cos();
        let y = self.y_center + radius * radians.sin();
        vec![x, y]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
