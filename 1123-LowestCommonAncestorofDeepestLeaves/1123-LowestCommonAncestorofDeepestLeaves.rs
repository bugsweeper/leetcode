// Last updated: 04.04.2025, 10:08:27
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

use std::cmp::Ordering;

fn lca_depth(
    node: &Option<Rc<RefCell<TreeNode>>>,
) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
    if node.is_none() {
        return (node.clone(), 0);
    }
    let node_ref = node.as_ref().unwrap().borrow();
    let (left_lca, left_depth) = lca_depth(&node_ref.left);
    let (right_lca, right_depth) = lca_depth(&node_ref.right);
    match left_depth.cmp(&right_depth) {
        Ordering::Greater => (left_lca, left_depth + 1),
        Ordering::Less => (right_lca, right_depth + 1),
        Ordering::Equal => (node.clone(), left_depth + 1),
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        lca_depth(&root).0
    }
}