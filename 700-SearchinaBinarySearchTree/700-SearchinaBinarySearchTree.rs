// Last updated: 21.04.2025, 11:49:10
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

pub fn search_bst(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if node.is_none() {
        return None;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    match val.cmp(&node_ref.val) {
        Ordering::Less => search_bst(&node_ref.left, val),
        Ordering::Greater => search_bst(&node_ref.right, val),
        Ordering::Equal => node.clone(),
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        search_bst(&root, val)
    }
}