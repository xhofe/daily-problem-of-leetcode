// https://leetcode-cn.com/problems/richest-customer-wealth/

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut ans=0;
        accounts.iter().for_each(|x|{
            ans = ans.max(x.iter().sum());
        });
        ans
    }
}