impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let mut ans = s.clone();
            let mut s = s;
            for _ in 0..s.len() {
                let c = s.remove(0);
                s.push(c);
                if s < ans {
                    ans = s.clone();
                }
            }
            ans
        } else {
            let mut bytes = s.bytes().collect::<Vec<_>>();
            bytes.sort();
            String::from_utf8(bytes.to_vec()).unwrap()
        }
    }
}
