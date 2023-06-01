impl Solution {
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        price.sort_unstable();
        let (mut left, mut right) = (0, price[price.len() - 1] - price[0]);
        let can_x = |x| {
            let mut num = 1;
            let mut last = price[0];
            for i in 1..price.len() {
                if price[i] - last >= x {
                    num += 1;
                    last = price[i];
                }
                if num >= k {
                    return true;
                }
            }
            num >= k
        };
        while left < right {
            let mid = (left + right + 1) / 2;
            if can_x(mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}
