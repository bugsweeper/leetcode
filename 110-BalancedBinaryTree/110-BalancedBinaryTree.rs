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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    let Some(root) = root else {
        return (0, true);
    };
    let root = Rc::into_inner(root).unwrap().into_inner();
    let max_left_depth = max_depth(root.left);
    if !max_left_depth.1 {
        // if left subtree is not balanced then all tree is not balanced
        // that's why skipping right subtree calculation
        return max_left_depth;
    }
    let max_right_depth = max_depth(root.right);
    let balanced = max_right_depth.1 && (max_right_depth.0 - max_left_depth.0).abs() < 2;
    (max_left_depth.0.max(max_right_depth.0) + 1, balanced)
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        max_depth(root).1
    }
}