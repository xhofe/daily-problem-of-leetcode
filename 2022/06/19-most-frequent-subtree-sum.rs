// https://leetcode.cn/problems/most-frequent-subtree-sum/

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
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn sum_tree(node: &Option<Rc<RefCell<TreeNode>>>, map: &mut std::collections::HashMap<i32,i32>) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let sum = node.val + sum_tree(&node.left, map) + sum_tree(&node.right,map);
                *map.entry(sum).or_default() += 1;
                sum
            } else {
                0
            }
        }
        let mut map = std::collections::HashMap::new();
        sum_tree(&root, &mut map);
        let mut ans = vec![];
        let mut cur = 0;
        for (k,v) in map {
            if v > cur {
                cur = v;
                ans.clear();
                ans.push(k);
            }else if v == cur {
                ans.push(k);
            }
        }
        ans
    }
}
