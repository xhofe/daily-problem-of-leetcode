impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![i as i32, j as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}
