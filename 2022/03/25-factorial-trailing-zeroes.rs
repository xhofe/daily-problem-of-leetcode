// https://leetcode-cn.com/problems/factorial-trailing-zeroes/

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let (mut ans, mut n) = (0, n);
        while n > 1 {
            n /= 5;
            ans += n;
        }
        ans
    }
}

// 2*5组成一个0，5的个数>2的个数，所以即求5的个数
// [1,n]中5的倍数 有n/5个，5^2的倍数有n/(5^2)个，以此类推，相加即可。