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

pub fn sum_numbers(node: Option<Rc<RefCell<TreeNode>>>, root_to_node: i32) -> i32 {
    let ref_node = node.as_ref().unwrap().borrow();
    let val = ref_node.val;
    let root_to_child = root_to_node * 10 + val;
    let mut result = 0;
    let left = ref_node.left.clone();
    let right = ref_node.right.clone();
    if left.is_none() && right.is_none() {
        result = root_to_child;
    } else {
        if left.is_some() {
            result += sum_numbers(left, root_to_child);
        }
        if right.is_some() {
            result += sum_numbers(right, root_to_child);
        }
    }
    result
}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_numbers(root, 0)
    }
}