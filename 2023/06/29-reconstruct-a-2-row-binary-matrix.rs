impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]; 2];
        for x in colsum {
            match x {
                0 => {
                    ans[0].push(0);
                    ans[1].push(0);
                }
                1 => {
                    if upper > lower {
                        ans[0].push(1);
                        ans[1].push(0);
                        upper -= 1;
                    } else {
                        ans[0].push(0);
                        ans[1].push(1);
                        lower -= 1;
                    }
                }
                2 => {
                    ans[0].push(1);
                    ans[1].push(1);
                    upper -= 1;
                    lower -= 1;
                }
                _ => unreachable!(),
            }
        }
        if upper != 0 || lower != 0 {
            return vec![];
        }
        ans
    }
}
