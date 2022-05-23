// https://leetcode.cn/problems/n-repeated-element-in-size-2n-array/

impl Solution {
  pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
      for i in 1..nums.len() {
          if nums[i] == nums[i-1] {
              return nums[i];
          }
      }
      if nums[0] == nums[2] || nums[0] == nums[3] {
          nums[0]
      }else{
          nums[1]
      }
  }
}