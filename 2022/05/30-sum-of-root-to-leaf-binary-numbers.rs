// https://leetcode.cn/problems/sum-of-root-to-leaf-binary-numbers/

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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Rc<RefCell<TreeNode>>, mut val: i32) -> i32 {
            val <<= 1;
            let node = node.borrow();
            val |= val + node.val;
            if node.left.is_none() && node.right.is_none() {
                return val;
            }
            let mut ans = 0;
            if node.left.is_some() {
                ans += dfs(node.left.as_ref().unwrap().clone(), val);
            }
            if node.right.is_some() {
                ans += dfs(node.right.as_ref().unwrap().clone(), val);
            }
            ans
        }
        dfs(root.unwrap(), 0)
    }
}
