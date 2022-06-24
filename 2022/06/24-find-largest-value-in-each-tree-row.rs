// https://leetcode.cn/problems/find-largest-value-in-each-tree-row/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        if root.is_none() {
            return ans;
        }
        let mut queue = std::collections::LinkedList::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut max = i32::MIN;
            let mut size = queue.len();
            while size > 0 {
                let node = queue.pop_front().unwrap();
                if node.borrow().val > max {
                    max = node.borrow().val;
                }
                if let Some(left) = node.borrow().left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    queue.push_back(right);
                }
                size -= 1;
            }
            ans.push(max);
        }
        ans
    }
}
