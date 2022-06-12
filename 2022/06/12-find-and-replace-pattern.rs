// https://leetcode.cn/problems/find-and-replace-pattern/

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let p_bytes = pattern.as_bytes();
        words
            .into_iter()
            .filter(|x| {
                let mut map1 = std::collections::HashMap::new();
                let mut map2 = std::collections::HashMap::new();
                let bytes = x.as_bytes();
                for i in 0..bytes.len() {
                    let b = bytes[i];
                    let p_b = p_bytes[i];
                    if (map1.contains_key(&b) && map1[&b] != p_b)
                        || (map2.contains_key(&p_b) && map2[&p_b] != b)
                    {
                        return false;
                    }
                    map1.entry(b).or_insert(p_b);
                    map2.entry(p_b).or_insert(b);
                }
                true
            })
            .collect()
    }
}
