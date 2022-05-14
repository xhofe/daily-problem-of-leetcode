// https://leetcode.cn/problems/find-all-duplicates-in-an-array/

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..nums.len() {
            let num = nums[i].abs();
            if nums[num as usize-1] > 0 {
                nums[num as usize-1] = -nums[num as usize-1];
            }else { res.push(num); }
        }
        res
    }
}