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
use std::collections::VecDeque;

struct BSTIterator {
    queue: Vec<Rc<RefCell<TreeNode>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let node = root.unwrap();
        let mut queue = Vec::new();
        queue.push(node);
        Self {
            queue
        }
    }
    
    fn next(&mut self) -> i32 {
        let mut node = self.queue.pop().unwrap();
        loop {
            let Some(left) = node.borrow_mut().left.take() else {
                break;
            };
            self.queue.push(node);
            node = left;
        }
        let mut ref_node = node.borrow_mut();
        if ref_node.right.is_some() {
            self.queue.push(ref_node.right.take().unwrap());
        }
        ref_node.val
    }
    
    fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */