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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_level = 0;
        let mut max_sum = i32::MIN;
        let mut level = 1;
        let mut queue = vec![root];
        while !queue.is_empty() {
            let mut sum = 0;
            let mut next_level = vec![];
            for node in queue {
                if let Some(node) = node {
                    sum += node.borrow().val;
                    if node.borrow().left.is_some() {
                        next_level.push(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        next_level.push(node.borrow().right.clone());
                    }
                }
            }
            if sum > max_sum {
                max_sum = sum;
                max_level = level;
            }
            queue = next_level;
            level += 1;
        }
        max_level as i32
    }
}
