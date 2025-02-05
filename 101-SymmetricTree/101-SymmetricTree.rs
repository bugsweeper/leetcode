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

fn are_symmetric(
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if left.is_none() && right.is_none() {
        return true;
    }
    let (Some(left), Some(right)) = (left, right) else {
        return false;
    };
    let (left, right) = (
        Rc::into_inner(left).unwrap().into_inner(),
        Rc::into_inner(right).unwrap().into_inner(),
    );
    left.val == right.val
        && are_symmetric(left.left, right.right)
        && are_symmetric(left.right, right.left)
}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };
        let root = Rc::into_inner(root).unwrap().into_inner();
        are_symmetric(root.left, root.right)
    }
}