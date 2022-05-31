// https://leetcode.cn/problems/Jf1JuT/
// 剑指 Offer II 114. 外星文字典

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph = std::collections::HashMap::new();
        let mut in_degree = std::collections::HashMap::new();
        for &ele in words[0].as_bytes() {
            in_degree.insert(ele, 0);
        }
        for w in words.windows(2) {
            let (s, t) = (&w[0], &w[1]);
            for &ele in t.as_bytes() {
                in_degree.entry(ele).or_insert(0);
            }
            let mut is = false;
            for (u, v) in s.bytes().zip(t.bytes()) {
                if u != v {
                    graph.entry(u).or_insert(vec![]).push(v);
                    *in_degree.entry(v).or_insert(0) += 1;
                    is = true;
                    break;
                }
            }
            if !is && s.len() > t.len() {
                return "".to_owned();
            }
        }
        let mut queue = in_degree
            .iter()
            .filter_map(|(&u, &d)| if d == 0 { Some(u) } else { None })
            .collect::<Vec<_>>();
        let mut len = queue.len();
        let mut i = 0;
        while i < len {
            let u = queue[i];
            if graph.contains_key(&u) {
                for &v in graph[&u].iter() {
                    let ind = in_degree.entry(v).or_default();
                    *ind -= 1;
                    if *ind == 0 {
                        queue.push(v);
                    }
                }
            }
            i += 1;
            len = queue.len();
        }
        if queue.len() == in_degree.len() {
            return queue.into_iter().map(|u| u as char).collect();
        }
        "".to_owned()
    }
}
