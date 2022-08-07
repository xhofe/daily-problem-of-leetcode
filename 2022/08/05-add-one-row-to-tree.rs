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
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 0 || depth == 1 {
            let mut t = TreeNode::new(val);
            if depth == 1 {
                t.left = root;
            } else {
                t.right = root;
            }
            return Some(Rc::new(RefCell::new(t)));
        }
        if root.is_some() && depth > 1 {
            let left = root.as_ref().unwrap().borrow().left.clone();
            let right = root.as_ref().unwrap().borrow().right.clone();
            root.as_ref().unwrap().borrow_mut().left =
                Self::add_one_row(left, val, if depth > 2 { depth - 1 } else { 1 });
            root.as_ref().unwrap().borrow_mut().right =
                Self::add_one_row(right, val, if depth > 2 { depth - 1 } else { 0 });
        }
        root
    }
}
