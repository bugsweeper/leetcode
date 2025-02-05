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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        let (Some(p), Some(q)) = (p, q) else {
            return false;
        };
        let (p, q) = (Rc::into_inner(p).unwrap().into_inner(), Rc::into_inner(q).unwrap().into_inner());
        p.val == q.val && Solution::is_same_tree(p.left, q.left) && Solution::is_same_tree(p.right, q.right)
    }
}