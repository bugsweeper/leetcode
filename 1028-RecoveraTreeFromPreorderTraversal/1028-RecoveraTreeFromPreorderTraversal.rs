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
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut depth = 0;
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::with_capacity(1000);
        let mut val = 0;
        // Additional '-' as end for last `val` to force add last node
        for &letter in traversal.as_bytes().iter().chain(std::iter::once(&b'-')) {
            if letter == b'-' {
                if val > 0 {
                    if depth < stack.len() {
                        stack.drain(depth..);
                    }
                    let node = Rc::new(RefCell::new(TreeNode::new(val)));
                    if let Some(parent) = stack.last_mut() {
                        let mut ref_mut= RefCell::borrow_mut(parent.as_mut().unwrap());
                        let TreeNode { left, right, .. } = ref_mut.deref_mut();
                        if left.is_none() {
                            *left = Some(node.clone());
                        } else {
                            *right = Some(node.clone());
                        }
                    }
                    stack.push(Some(node));
                    val = 0;
                    depth = 0;
                }
                depth += 1;
            } else {
                val = val * 10 + (letter & 0xf) as i32;
            }
        }
        stack.first_mut().unwrap().take()
    }
}