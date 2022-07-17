// https://leetcode.cn/problems/array-nesting/

impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] == -1 {
                continue;
            }
            let mut count = 0;
            let mut j = i;
            while nums[j] != -1 {
                let tmp = nums[j];
                nums[j] = -1;
                j = tmp as usize;
                count += 1;
            }
            ans = ans.max(count);
        }
        ans
    }
}
