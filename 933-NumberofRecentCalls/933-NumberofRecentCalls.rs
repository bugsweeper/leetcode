// Last updated: 14.05.2025, 16:46:55
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

fn is_same(node: &Option<Rc<RefCell<TreeNode>>>, value: i32) -> bool {
    if node.is_none() {
        return true;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    node_ref.val == value && is_same(&node_ref.left, value) && is_same(&node_ref.right, value)
}

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root_ref = root.as_ref().unwrap().borrow();
        is_same(&root_ref.left, root_ref.val) && is_same(&root_ref.right, root_ref.val)
    }
}