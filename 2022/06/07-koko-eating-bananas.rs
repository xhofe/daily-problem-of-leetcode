// https://leetcode.cn/problems/koko-eating-bananas/

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let piles = piles.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let h = h as i64;
        let mut left = 1;
        let mut right = piles.iter().sum::<i64>() as i64;
        fn can_eat_all_piles(piles: &Vec<i64>, h: i64, speed: i64) -> bool {
            let mut sum = 0;
            for pile in piles {
                sum += pile / speed;
                if pile % speed != 0 {
                    sum += 1;
                }
            }
            sum <= h
        }
        while left < right {
            let mid = left + ((right - left) >> 1);
            if can_eat_all_piles(&piles, h, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}
