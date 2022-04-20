// https://leetcode-cn.com/problems/longest-absolute-file-path/

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        fn count_tab(line: &str) -> usize {
            let mut ans = 0;
            for &x in line.as_bytes() {
                if x == b'\t' {
                    ans += 1;
                } else { break; }
            }
            ans
        }
        let mut ans = 0;
        let lines = input.split("\n");
        let mut stack = Vec::new();
        lines.for_each(|line| {
            let tab_count = count_tab(line);
            if tab_count < stack.len() {
                stack.truncate(tab_count);
            }
            let is_file = line.contains(".");
            if is_file {
                ans = ans.max(stack.join("/").len() + line.len() - tab_count + if stack.is_empty() { 0 } else { 1 });
            } else {
                stack.push(&line[tab_count..])
            }
        });
        ans as i32
    }
}
