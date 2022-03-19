// https://leetcode-cn.com/problems/construct-string-from-binary-tree/

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
  pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut res = String::new();
    if let Some(node) = root {
      res.push_str(&node.borrow().val.to_string());
      let left = Self::tree2str(node.borrow().left.clone());
      let right = Self::tree2str(node.borrow().right.clone());
      if left.len() > 0 || right.len() > 0 {
        res.push_str(&format!("({})", left));
      }
      if right.len() > 0 {
        res.push_str(&format!("({})", right));
      }
    }
    res
  }
}
