// https://leetcode-cn.com/problems/subarray-product-less-than-k/

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 || k == 1 { return 0; }
        let mut left = 0;
        let mut product = 1;
        let mut ans = 0;
        for i in 0..nums.len() {
            product *= nums[i];
            while product >= k {
                product /= nums[left];
                left += 1;
            }
            ans += (i + 1 - left) as i32
        }
        ans
    }   
}