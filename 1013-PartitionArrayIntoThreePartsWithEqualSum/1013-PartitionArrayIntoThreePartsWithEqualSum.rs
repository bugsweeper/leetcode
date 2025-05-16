// Last updated: 16.05.2025, 23:00:16
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

fn sum_root_to_leaf(node: &Option<Rc<RefCell<TreeNode>>>, prefix: i32) -> i32 {
    let node_ref = node.as_ref().unwrap().borrow();
    let value = (prefix << 1) + node_ref.val;
    match (node_ref.left.is_some(), node_ref.right.is_some()) {
        (true, true) => sum_root_to_leaf(&node_ref.left, value) + sum_root_to_leaf(&node_ref.right, value),
        (true, _) => sum_root_to_leaf(&node_ref.left, value),
        (_, true) => sum_root_to_leaf(&node_ref.right, value),
        _ => value
    }
}

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_root_to_leaf(&root, 0)
    }
}