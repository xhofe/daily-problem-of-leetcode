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
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn f(
            node: Option<Rc<RefCell<TreeNode>>>,
            to_delete: &Vec<i32>,
        ) -> (
            Option<Rc<RefCell<TreeNode>>>,
            Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) {
            if let Some(node) = node {
                let mut node_bm = node.borrow_mut();
                let (l, r) = (node_bm.left.take(), node_bm.right.take());
                let (l, r) = (f(l, to_delete), f(r, to_delete));
                if to_delete.contains(&node_bm.val) {
                    let mut ans = vec![];
                    ans.extend(l.1);
                    ans.extend(r.1);
                    if l.0.is_some() {
                        ans.push(l.0);
                    }
                    if r.0.is_some() {
                        ans.push(r.0);
                    }
                    (None, ans)
                } else {
                    node_bm.left = l.0;
                    node_bm.right = r.0;
                    let mut ans = vec![];
                    ans.extend(l.1);
                    ans.extend(r.1);
                    (Some(node.clone()), ans)
                }
            } else {
                (None, vec![])
            }
        }
        let (node, mut nodes) = f(root, &to_delete);
        if node.is_some() {
            nodes.push(node);
        }
        nodes
    }
}
