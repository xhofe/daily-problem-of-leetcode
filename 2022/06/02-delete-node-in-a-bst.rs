// https://leetcode.cn/problems/delete-node-in-a-bst/

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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut cur,mut cur_parent) = (root.clone(),None);
        while cur.is_some() && cur.as_ref().unwrap().borrow().val != key {
            cur_parent = cur.clone();
            let val = cur.as_ref().unwrap().borrow().val;
            if key < val {
                let left = cur.as_ref().unwrap().borrow().left.clone();
                cur = left;
            } else {
                let right = cur.as_ref().unwrap().borrow().right.clone();
                cur = right;
            }
        }
        if cur.is_none() {
            return root;
        }
        if cur.as_ref().unwrap().borrow().left.is_none() && cur.as_ref().unwrap().borrow().right.is_none() {
            cur = None
        }else if cur.as_ref().unwrap().borrow().right.is_none() {
            let left = cur.as_ref().unwrap().borrow().left.clone();
            cur = left;
        }else if cur.as_ref().unwrap().borrow().left.is_none() {
            let right = cur.as_ref().unwrap().borrow().right.clone();
            cur = right;
        }else {
            let (mut remove, mut remove_parent) = (cur.as_ref().unwrap().borrow().right.clone(),cur.clone());
            while remove.as_ref().unwrap().borrow().left.is_some() {
                remove_parent = remove.clone();
                let left = remove.as_ref().unwrap().borrow().left.clone();
                remove = left;
            }
            if remove_parent.as_ref().unwrap().borrow().val == cur.as_ref().unwrap().borrow().val {
                remove_parent.as_ref().unwrap().borrow_mut().right = remove.as_ref().unwrap().borrow().right.clone();
            }else {
                remove_parent.as_ref().unwrap().borrow_mut().left = remove.as_ref().unwrap().borrow().right.clone();
            }
            remove.as_ref().unwrap().borrow_mut().right = cur.as_ref().unwrap().borrow().right.clone();
            remove.as_ref().unwrap().borrow_mut().left = cur.as_ref().unwrap().borrow().left.clone();
            cur = remove;
        }
        if cur_parent.is_none() {
            return cur;
        }
        if cur_parent.as_ref().unwrap().borrow().left.is_some() && cur_parent.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val == key {
            cur_parent.as_ref().unwrap().borrow_mut().left = cur;
        }else {
            cur_parent.as_ref().unwrap().borrow_mut().right = cur;
        }
        root
    }
}
