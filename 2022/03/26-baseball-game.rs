// https://leetcode-cn.com/problems/baseball-game/

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack = vec![];
        ops.iter().for_each(|x|{
            match x.as_str() {
                "+"=>{
                    stack.push(stack.last().unwrap()+stack[stack.len()-2]);
                }
                "D"=>{
                    stack.push(stack.last().unwrap()*2)
                }
                "C"=>{
                    stack.pop();
                }
                _=>{
                    stack.push(x.parse::<i32>().unwrap());
                }
            }
        });
        stack.iter().sum()
    }
}