// https://leetcode.cn/problems/stickers-to-spell-word/

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let m = target.len();
        let mut f = vec![-1; 1 << m];
        f[0] = 0;
        fn dp(mask: usize, f: &mut Vec<i32>, stickers: &Vec<String>, target: &String) -> i32 {
            if f[mask] != -1 {
                return f[mask];
            }
            f[mask] = target.len() as i32 + 1;
            for x in stickers {
                let mut left = mask;
                let mut cnt = [0; 26];
                for c in x.bytes() {
                    cnt[(c - b'a') as usize] += 1;
                }
                for (i, c) in target.bytes().enumerate() {
                    if mask >> i & 1 == 1 && cnt[(c - b'a') as usize] > 0 {
                        cnt[(c - b'a') as usize] -= 1;
                        left ^= 1 << i;
                    }
                }
                if left < mask {
                    f[mask] = f[mask].min(dp(left, f, stickers, target) + 1);
                }
            }
            f[mask]
        }
        let ans = dp((1 << m) - 1, &mut f, &stickers, &target);
        if ans < m as i32 {
            return ans;
        }
        -1
    }
}