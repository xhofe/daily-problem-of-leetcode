impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowel = vec![b'a', b'e', b'i', b'o', b'u']
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let pre_sum = words
            .into_iter()
            .map(|x| vowel.contains(&x.as_bytes()[0]) && vowel.contains(&x.as_bytes()[x.len() - 1]))
            .fold(vec![0], |mut f, x| {
                f.push(f[f.len() - 1] + if x { 1 } else { 0 });
                f
            });
        queries
            .into_iter()
            .map(|x| pre_sum[x[1] as usize + 1] - pre_sum[x[0] as usize])
            .collect()
    }
}
