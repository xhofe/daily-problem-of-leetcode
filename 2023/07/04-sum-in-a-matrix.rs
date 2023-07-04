impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.iter_mut().for_each(|row| row.sort());
        (0..nums[0].len())
            .map(|i| nums.iter().map(|row| row[i]).max().unwrap())
            .sum()
    }
}
