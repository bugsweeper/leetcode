// Last updated: 12.05.2025, 23:13:14
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

pub fn range_sum_bst(node: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if node.is_none() {
        return 0;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    if node_ref.val < low {
        return range_sum_bst(&node_ref.right, low, high);
    }
    if node_ref.val > high {
        return range_sum_bst(&node_ref.left, low, high);
    }
    range_sum_bst(&node_ref.left, low, high) + node_ref.val + range_sum_bst(&node_ref.right, low, high)
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        range_sum_bst(&root, low, high)
    }
}