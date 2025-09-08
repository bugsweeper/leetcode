// Last updated: 08.09.2025, 10:18:56
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

fn evaluate_tree(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let node_ref = node.as_ref().unwrap().borrow();
    match node_ref.val {
        0 => return false,
        1 => return true,
        _ => {}
    }
    let left = evaluate_tree(&node_ref.left);
    match (left, node_ref.val) {
        (true, 2) | (false, 3) => return left,
        _ => evaluate_tree(&node_ref.right),
    }
}

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        evaluate_tree(&root)
    }
}