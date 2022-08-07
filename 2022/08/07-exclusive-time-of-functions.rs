impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut stack = vec![];
        let mut res = vec![0; n as usize];
        for log in logs {
            let split = log.split(":").collect::<Vec<&str>>();
            let id = split[0].parse::<i32>().unwrap();
            let time = split[2].parse::<i32>().unwrap();
            if split[1] == "start" {
                stack.push((id, time));
            } else {
                let (id, start) = stack.pop().unwrap();
                res[id as usize] += time - start + 1;
                if let Some((id, _)) = stack.last() {
                    res[*id as usize] -= time - start + 1;
                }
            }
        }
        res
    }
}
