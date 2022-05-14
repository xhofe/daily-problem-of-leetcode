// https://leetcode.cn/problems/delete-columns-to-make-sorted/

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let matrix = strs.into_iter().map(|x| {
            x.bytes().collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let mut ans = 0;
        for i in 0..matrix[0].len() {
            for j in 0..matrix.len() - 1 {
                if matrix[j][i] > matrix[j + 1][i] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}