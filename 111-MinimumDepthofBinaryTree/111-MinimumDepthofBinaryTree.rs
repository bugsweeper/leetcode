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
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        let root = Rc::into_inner(root).unwrap().into_inner();
        match (root.left.is_some(), root.right.is_some()) {
            (false, false) => 1,
            (false, true) => Solution::min_depth(root.right) + 1,
            (true, false) => Solution::min_depth(root.left) + 1,
            (true, true) => Solution::min_depth(root.left).min(Solution::min_depth(root.right)) + 1,
        }
    }
}