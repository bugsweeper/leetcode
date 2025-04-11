// Last updated: 11.04.2025, 16:42:05
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

fn merge(
    destination: &mut Option<Rc<RefCell<TreeNode>>>,
    source: &mut Option<Rc<RefCell<TreeNode>>>,
) {
    if destination.is_none() {
        std::mem::swap(destination, source);
        return;
    }
    if source.is_none() {
        return;
    }
    let mut destination_ref = destination.as_mut().unwrap().borrow_mut();
    let mut source_ref = source.as_mut().unwrap().borrow_mut();
    destination_ref.val += source_ref.val;
    merge(&mut destination_ref.left, &mut source_ref.left);
    merge(&mut destination_ref.right, &mut source_ref.right);
}

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root1 = root1;
        let mut root2 = root2;
        merge(&mut root1, &mut root2);
        root1
    }
}