impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut set = arr
            .iter()
            .map(|&x| x)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        set.sort();
        let dict = set
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i + 1))
            .collect::<std::collections::HashMap<_,_>>();
        arr.into_iter().map(|x| *dict.get(&x).unwrap() as i32).collect()
    }
}
