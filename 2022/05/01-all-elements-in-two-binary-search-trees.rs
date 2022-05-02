// https://leetcode-cn.com/problems/all-elements-in-two-binary-search-trees/

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
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn in_order(node: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
            if let Some(node) = node {
                let node = node.borrow();
                in_order(&node.left, vec);
                vec.push(node.val);
                in_order(&node.right, vec);
            }
        }
        let mut ret = Vec::new();
        in_order(&root1, &mut ret);
        in_order(&root2, &mut ret);
        ret.sort();
        ret
    }
}