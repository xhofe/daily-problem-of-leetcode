// https://leetcode-cn.com/problems/projection-area-of-3d-shapes/

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter().map(|x|{
            x.iter().filter(|&&y|{
                y!=0
            }).count() as i32
        }).sum::<i32>() + grid.iter().map(|x|{
            x.iter().max().unwrap()
        }).sum::<i32>() + {
            let mut ans = 0;
            for i in 0..grid.len() {
                let mut max = 0;
                for j in 0..grid.len() {
                    max = max.max(grid[j][i]);
                }
                ans+=max;
            }
            ans
        }
    }
}