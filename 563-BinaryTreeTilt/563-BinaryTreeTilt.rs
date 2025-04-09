// Last updated: 09.04.2025, 12:12:21
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

fn sum_and_tilts(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if node.is_none() {
        return (0, 0);
    }
    let node_ref = node.as_ref().unwrap().borrow();
    let (left_sum, left_tilts) = sum_and_tilts(&node_ref.left);
    let (right_sum, right_tilts) = sum_and_tilts(&node_ref.right);
    (left_sum + right_sum + node_ref.val, left_tilts + right_tilts + (left_sum - right_sum).abs())
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_and_tilts(&root).1
    }
}