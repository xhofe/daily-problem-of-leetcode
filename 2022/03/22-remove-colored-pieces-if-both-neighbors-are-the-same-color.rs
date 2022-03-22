// https://leetcode-cn.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color/

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let (mut cur, mut cnt) = (' ', 0);
        let mut count = 0;
        for c in colors.chars() {
            if c != cur {
                cur = c;
                cnt = 1;
            } else {
                cnt += 1;
                if cnt >= 3 {
                    count += if cur == 'A' { 1 } else { -1 }
                }
            }
        }
        count > 0
    }
}