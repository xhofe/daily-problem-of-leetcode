// https://leetcode.cn/problems/binary-tree-pruning/

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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let left = Self::prune_tree(r.borrow_mut().left.take());
            let right = Self::prune_tree(r.borrow_mut().right.take());
            if r.borrow().val == 0 && left.is_none() && right.is_none() {
                return None;
            }
            r.borrow_mut().left = left;
            r.borrow_mut().right = right;
            Some(r)
        } else {
            None
        }
    }
}
