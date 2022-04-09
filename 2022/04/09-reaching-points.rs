// https://leetcode-cn.com/problems/reaching-points/

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let (mut tx, mut ty) = (tx, ty);
        while tx >= sx && ty >= sy {
            if tx == sx && ty == sy {
                return true;
            }
            if tx > ty {
                tx -= ty * 1.max((tx - sx) / ty);
            } else {
                ty -= tx * 1.max((ty - sy) / tx);
            }
        }
        false
    }
}

// 倒推