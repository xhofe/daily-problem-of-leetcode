// https://leetcode.cn/problems/length-of-longest-fibonacci-subsequence/

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let index = arr
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect::<std::collections::HashMap<_, _>>();
        let mut lagest = std::collections::HashMap::new();
        let n = arr.len();
        let mut ans = 0;
        for i in 0..n {
            for j in 0..i {
                let target = arr[i] - arr[j];
                if let Some(&k) = index.get(&target) {
                    if k < j {
                        let tmp = *lagest.get(&(j * n + k)).unwrap_or(&2) + 1;
                        ans = ans.max(tmp);
                        lagest.insert(i * n + j, tmp);
                    }
                }
            }
        }
        ans
    }
}
