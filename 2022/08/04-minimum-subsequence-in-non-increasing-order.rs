impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let sum = nums.iter().sum::<i32>();
        let mut nums = nums;
        nums.sort();
        nums.reverse();
        let mut tmp = 0;
        let mut idx = 0;
        let mut ans = nums
            .iter()
            .enumerate()
            .take_while(|&x| {
                tmp += *x.1;
                idx = x.0;
                tmp * 2 <= sum
            })
            .map(|x| *x.1)
            .collect::<Vec<_>>();
        ans.push(nums[idx]);
        ans
    }
}
