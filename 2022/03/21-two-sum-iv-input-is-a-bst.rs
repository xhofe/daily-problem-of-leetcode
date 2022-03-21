// https://leetcode-cn.com/problems/two-sum-iv-input-is-a-bst/

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
use std::collections::HashSet;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::new();
        return Self::dfs(&root, k, &mut set);
    }
    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>) -> bool {
        match node {
            None => false,
            Some(x) => {
                if set.contains(&(k - x.borrow().val)) {
                    true
                } else {
                    set.insert(x.borrow().val);
                    Self::dfs(&x.borrow().left, k, set) || Self::dfs(&x.borrow().right, k, set)
                }
            }
        }
    }
}