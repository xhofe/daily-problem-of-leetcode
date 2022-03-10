// https://leetcode-cn.com/problems/smallest-rotation-with-highest-score/

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        // nums_i <= (i-k+n)%n  0<=(i-k+n)%n<=n-1
        // k<=(i-nums_i+n)%n  k<=(i+n)%n k>=(i+1)%n
        // k (- [(i+1)%n,(i-nums_i+n)%n]
        let n = nums.len();
        let mut diff = vec![0; n+1];
        for i in 0..nums.len() {
            let left = (i+1)%n;
            let right = (i-(nums[i] as usize)+n)%n;
            diff[left]+=1;
            diff[right+1]-=1;
            if left > right {
                diff[0]+=1;
                diff[n]-=1;
            }
        }
        let mut ans = 0;
        let mut tmp = 0;
        let mut sum = 0;
        for k in 0..n {
            sum += diff[k];
            if sum > tmp {
                tmp = sum;
                ans = k;
            }
        }
        ans as i32
    }
}