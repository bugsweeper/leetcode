// Last updated: 13.05.2025, 15:12:31
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

// returns max independent path and max path to current node
pub fn max_path_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    let node_ref = node.as_ref().unwrap().borrow();
    match (node_ref.left.is_some(), node_ref.right.is_some()) {
        (false, false) => (node_ref.val, node_ref.val),
        (false, true) => {
            let (max, upside) = max_path_sum(&node_ref.right);
            (max.max(node_ref.val + upside.max(0)), node_ref.val + upside.max(0))
        }
        (true, false) => {
            let (max, upside) = max_path_sum(&node_ref.left);
            (max.max(node_ref.val + upside.max(0)), node_ref.val + upside.max(0))
        }
        (true, true) => {
            let (left_max, left_upside) = max_path_sum(&node_ref.left);
            let (right_max, right_upside) = max_path_sum(&node_ref.right);
            (left_max.max(right_max).max(node_ref.val + left_upside.max(0) + right_upside.max(0)), node_ref.val + left_upside.max(right_upside).max(0))
        }
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_path_sum(&root).0
    }
}