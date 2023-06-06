impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len() - 1).for_each(|i| {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        });
        let (mut l, r): (Vec<_>, Vec<_>) = nums.into_iter().partition(|&x| x != 0);
        l.extend(r);
        l
    }
}
