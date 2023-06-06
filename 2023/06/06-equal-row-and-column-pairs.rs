impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut map1 = std::collections::HashMap::new();
        let mut map2 = std::collections::HashMap::new();
        for row in grid.iter() {
            *map1.entry(row).or_insert(0) += 1;
        }
        for i in 0..grid.len() {
            let col = (0..grid.len()).map(|j| grid[j][i]).collect::<Vec<_>>();
            *map2.entry(col).or_insert(0) += 1;
        }
        for (k, v) in map2.iter() {
            ans += map1.get(k).unwrap_or(&0) * v;
        }
        ans
    }
}
