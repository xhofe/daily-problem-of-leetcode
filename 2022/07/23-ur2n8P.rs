// https://leetcode.cn/problems/ur2n8P/

impl Solution {
    pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut deque = std::collections::VecDeque::new();
        let mut v = vec![0; n + 1];
        let mut g = vec![vec![]; n + 1];
        for sequence in sequences {
            for i in 0..(sequence.len() - 1) {
                v[sequence[i + 1] as usize] += 1;
                g[sequence[i] as usize].push(sequence[i + 1]);
            }
        }
        let mut ptr = 0;
        for i in 1..=n {
            if v[i] == 0 {
                deque.push_back(i as i32);
            }
        }
        while !deque.is_empty() {
            if deque.len() > 1 {
                return false;
            }
            let cur = deque.pop_front().unwrap();
            if nums[ptr as usize] != cur {
                return false;
            }
            ptr += 1;
            for next in g[cur as usize].iter() {
                v[*next as usize] -= 1;
                if v[*next as usize] == 0 {
                    deque.push_back(*next);
                }
            }
        }
        ptr == n
    }
}
