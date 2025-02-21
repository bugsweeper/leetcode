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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut stack = Vec::with_capacity(1000);
        let mut node = root.clone();
        loop {
            let mut left =  node.as_mut().unwrap().borrow_mut().left.take();
            if left.is_some() {
                std::mem::swap(&mut node.as_mut().unwrap().borrow_mut().right, &mut left);
                if left.is_some() {
                    stack.push(left);
                }
            } else if node.as_ref().unwrap().borrow().right.is_none() {
                if stack.is_empty() {
                    return;
                }
                let _ = std::mem::replace(&mut node.as_mut().unwrap().borrow_mut().right, stack.pop().unwrap());
            }
            let next_node = node.as_ref().unwrap().borrow().right.clone();
            node = next_node;
        }
    }
}