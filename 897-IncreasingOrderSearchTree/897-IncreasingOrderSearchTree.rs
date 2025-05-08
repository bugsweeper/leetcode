// Last updated: 08.05.2025, 15:23:49
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
type NodeLink = Option<Rc<RefCell<TreeNode>>>;

pub fn increasing_bst(node: NodeLink) -> (NodeLink, NodeLink) {
    let mut node = node;
    let mut node_ref = node.as_mut().unwrap().borrow_mut();
    let (mut left, mut right) = (node_ref.left.take(), node_ref.right.take());
    let last;
    if right.is_some() {
        (node_ref.right, last) = increasing_bst(right);
        std::mem::drop(node_ref);
    } else {
        std::mem::drop(node_ref);
        last = node.clone();
    }
    if left.is_some() {
        let (top, mut left_last) = increasing_bst(left);
        left_last.as_mut().unwrap().borrow_mut().right = node;
        (top, last)
    } else {
        (node, last)
    }
}

impl Solution {
    pub fn increasing_bst(root: NodeLink) -> NodeLink {
        increasing_bst(root).0
    }
}