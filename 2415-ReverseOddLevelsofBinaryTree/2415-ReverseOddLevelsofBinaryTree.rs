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

fn swap_recursive(
    left: &Option<Rc<RefCell<TreeNode>>>,
    right: &Option<Rc<RefCell<TreeNode>>>,
    mut level: i32,
) {
    let mut left = left.as_ref().unwrap().borrow_mut();
    let mut right = right.as_ref().unwrap().borrow_mut();
    if level % 2 == 1 {
        std::mem::swap(&mut left.val, &mut right.val);
    }
    if left.left.is_none() {
        return;
    }
    level += 1;
    swap_recursive(&left.left, &right.right, level);
    swap_recursive(&left.right, &right.left, level);
}

impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        {
            let ref_root = root.as_ref().unwrap().borrow_mut();
            if ref_root.left.is_some() {
                swap_recursive(&ref_root.left, &ref_root.right, 1);
            }
        }

        root
    }
}