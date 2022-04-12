// https://leetcode-cn.com/problems/number-of-lines-to-write-string/

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let (mut rows,mut cols) = (1,0);
        s.bytes().for_each(|x|{
            let width = widths[(x-b'a') as usize];
            cols += width;
            if cols > 100 {
                cols = width;
                rows+=1;
            }
        });
        vec![rows,cols]
    }
}