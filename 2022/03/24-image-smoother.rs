// https://leetcode-cn.com/problems/image-smoother/

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0;img[0].len()];img.len()];
        for i in 0..img.len() {
            for j in 0..img[0].len() {
                let mut count = 0;
                for k in i.saturating_sub(1)..(i + 2).min(img.len()) {
                    for l in j.saturating_sub(1)..(j+2).min(img[0].len()) {
                        res[i][j]+=img[k][l];count+=1;
                    }
                }
                res[i][j]/=count;
            }
        }
        res
    }
}