// Last updated: 21.04.2025, 10:10:45
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

pub fn search(node: &Option<Rc<RefCell<TreeNode>>>, min: i32) -> i32 {
    let node_ref = node.as_ref().unwrap().borrow();
    if node_ref.left.is_none() {
        return if node_ref.val == min { -1 } else { node_ref.val }
    }
    match (search(&node_ref.left, min), search(&node_ref.right, min)) {
        (-1, -1) => -1,
        (-1, second) | (second, -1) => second,
        (val1, val2) => val1.min(val2),
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root_ref = root.as_ref().unwrap().borrow();
        search(&root, root_ref.val)
    }
}