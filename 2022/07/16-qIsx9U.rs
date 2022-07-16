// https://leetcode.cn/problems/qIsx9U/

struct MovingAverage {
    size: i32,
    nums: Vec<i32>,
    sum: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        Self {
            size,
            nums: Vec::new(),
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.nums.push(val);
        self.sum += val as i64;
        if self.nums.len() > self.size as usize {
            self.sum -= self.nums.remove(0) as i64;
        }
        (self.sum as f64) / (self.nums.len() as f64)
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */
