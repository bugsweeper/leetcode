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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root) = root else {
            return false;
        };
        let root = Rc::into_inner(root).unwrap().into_inner();
        if target_sum == root.val && root.left.is_none() && root.right.is_none() {
            return true;
        }
        let child_sum = target_sum - root.val;
        Solution::has_path_sum(root.left, child_sum) || Solution::has_path_sum(root.right, child_sum)
    }
}