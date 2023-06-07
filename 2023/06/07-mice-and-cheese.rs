impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut diff = reward1
            .iter()
            .zip(reward2.iter())
            .map(|(&x, &y)| (x, y, x - y))
            .collect::<Vec<_>>();
        diff.sort_unstable_by_key(|x| -x.2);
        diff.iter().take(k as usize).map(|x| x.0).sum::<i32>()
            + diff.iter().skip(k as usize).map(|x| x.1).sum::<i32>()
    }
}
