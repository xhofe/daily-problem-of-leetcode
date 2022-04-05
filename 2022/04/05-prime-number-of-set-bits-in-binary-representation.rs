// https://leetcode-cn.com/problems/prime-number-of-set-bits-in-binary-representation/

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        fn is_prime(num: usize) -> bool {
            for i in if num==2 {3 }else { 2 }..=(num as f64).sqrt() as usize {
                if num % i == 0 {
                    return false;
                }
            }
            return true;
        }
        let mut isprime = vec![false;32];
        for i in 2..32 {
            isprime[i] = is_prime(i);
        }
        let mut ans = 0;
        for i in left..=right {
            let count = i.count_ones();
            ans += if isprime[count as usize] {1 }else { 0 }
        }
        ans
    }
}