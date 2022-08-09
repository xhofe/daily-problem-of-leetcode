impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        min = nums
            .into_iter()
            .reduce(|acc, x| {
                min = min.min(acc);
                acc + x
            })
            .unwrap()
            .min(min);
        if min > 0 {
            1
        } else {
            min.abs() + 1
        }
    }
}
