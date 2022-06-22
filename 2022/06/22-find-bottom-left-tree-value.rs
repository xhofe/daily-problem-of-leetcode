// https://leetcode.cn/problems/find-bottom-left-tree-value/

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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_height = 0;
        let mut ans = 0;
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, cur_h: i32, max_h: &mut i32, ans: &mut i32) {
            if let Some(node) = node {
                inorder(&node.borrow().left, cur_h + 1, max_h, ans);
                if cur_h > *max_h {
                    *max_h = cur_h;
                    *ans = node.borrow().val;
                }
                inorder(&node.borrow().right, cur_h + 1, max_h, ans);
            }
        }
        inorder(&root, 1, &mut max_height, &mut ans);
        ans
    }
}
