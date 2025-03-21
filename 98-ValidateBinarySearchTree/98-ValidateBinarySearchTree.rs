// Last updated: 21.03.2025, 16:33:41
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

fn get_bounds(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32)> {
    let node_ref = node.as_ref().unwrap().borrow();
    let min = if node_ref.left.is_some() {
        let (left_min, left_max) = get_bounds(&node_ref.left)?;
        if left_max >= node_ref.val {
            return None;
        }
        left_min
    } else {
        node_ref.val
    };
    let max = if node_ref.right.is_some() {
        let (right_min, right_max) = get_bounds(&node_ref.right)?;
        if right_min <= node_ref.val {
            return None;
        }
        right_max
    } else {
        node_ref.val
    };
    Some((min, max))
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        get_bounds(&root).is_some()
    }
}