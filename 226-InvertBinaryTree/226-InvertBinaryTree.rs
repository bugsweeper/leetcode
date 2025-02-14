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
use std::ops::DerefMut;

fn swap_branches(ref_node: &mut Option<Rc<RefCell<TreeNode>>>) {
    if ref_node.is_none() {
        return;
    }
    let mut node = ref_node.as_mut().unwrap().borrow_mut();
    // Split into two separate subbranches
    let TreeNode { left, right, .. } = node.deref_mut();
    std::mem::swap(left, right);
    swap_branches(left);
    swap_branches(right);
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        swap_branches(&mut root);
        root
    }
}