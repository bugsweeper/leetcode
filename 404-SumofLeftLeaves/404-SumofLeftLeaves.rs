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

fn traverse_nodes(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
    if node.is_none() {
        return 0;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    if is_left && node_ref.left.is_none() && node_ref.right.is_none() {
        node_ref.val
    } else {
        traverse_nodes(&node_ref.left, true) + traverse_nodes(&node_ref.right, false)
    }
}

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        traverse_nodes(&root, false)
    }
}