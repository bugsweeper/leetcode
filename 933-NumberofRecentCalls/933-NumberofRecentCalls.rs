// Last updated: 15.05.2025, 13:43:23
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

// returns depth and parent
fn search(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, parent_value: i32, value: i32) -> Option<(usize, i32)> {
    let node_ref = node.as_ref()?.borrow();
    if node_ref.val == value {
        return Some((depth, parent_value));
    }
    search(&node_ref.left, depth + 1, node_ref.val, value).or(search(&node_ref.right, depth + 1, node_ref.val, value))
}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let (x_depth, x_parent) = search(&root, 0, 0, x).unwrap();
        let (y_depth, y_parent) = search(&root, 0, 0, y).unwrap();
        x_depth == y_depth && x_parent != y_parent
    }
}