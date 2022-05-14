// https://leetcode.cn/problems/serialize-and-deserialize-bst/

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

struct Codec {}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut arr = Vec::new();
        fn post_order(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<String>) {
            if node.is_none() { return; }
            post_order(node.as_ref().unwrap().borrow().left.clone(), arr);
            post_order(node.as_ref().unwrap().borrow().right.clone(), arr);
            arr.push(node.as_ref().unwrap().borrow().val.to_string());
        }
        post_order(root, &mut arr);
        let ans = arr.join(" ");
        return ans;
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() { return None; }
        let mut arr = data.split_whitespace().collect::<Vec<_>>();
        fn g(arr: &mut Vec<&str>, lower: i32, upper: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if arr.is_empty() { return None; }
            let val = arr.last().unwrap().parse::<i32>().unwrap();
            if val < lower || val > upper { return None; }
            arr.pop();
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                right: g(arr, val, upper),
                left: g(arr, lower, val),
            })));
        }
        g(&mut arr, i32::MIN, i32::MAX)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */