impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut set = std::collections::HashSet::new();
        while !nums.is_empty() {
            let (a, b) = (nums[0], nums[nums.len() - 1]);
            set.insert(a + b);
            nums = nums[1..nums.len() - 1].to_vec();
        }
        set.len() as i32
    }
}
