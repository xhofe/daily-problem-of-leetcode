// https://leetcode.cn/problems/complete-binary-tree-inserter/

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

use std::cell::RefCell;
use std::rc::Rc;
struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    queue: std::collections::LinkedList<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut queue = std::collections::LinkedList::new();
        let mut tmp = std::collections::LinkedList::new();
        let mut node = root.clone().unwrap();
        tmp.push_back(node);
        while !tmp.is_empty() {
            let cur = tmp.pop_front().unwrap();
            if cur.borrow().left.is_some() {
                tmp.push_back(cur.borrow().left.clone().unwrap());
            }
            if cur.borrow().right.is_some() {
                tmp.push_back(cur.borrow().right.clone().unwrap());
            } else {
                queue.push_back(cur);
            }
        }
        Self { root, queue }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let cur = self.queue.pop_front().unwrap();
        let ans = cur.borrow().val;
        if cur.borrow().left.is_some() {
            cur.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            self.queue.push_back(cur.borrow().right.clone().unwrap());
        } else {
            cur.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            self.queue.push_back(cur.borrow().left.clone().unwrap());
            self.queue.push_front(cur);
        }
        ans
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(val);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */
