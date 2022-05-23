// https://leetcode.cn/problems/univalued-binary-tree/

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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(val: i32, node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(node) = node {
                let node = node.borrow();
                if node.val != val {
                    return false;
                }
                return dfs(val, &node.left) && dfs(val, &node.right);
            }
            true
        }
        dfs(root.as_ref().unwrap().borrow().val, &root)
    }
}
