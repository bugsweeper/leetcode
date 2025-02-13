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

fn get_left_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> u32 {
    if node.is_none() {
        return 0;
    }
    get_left_depth(&node.as_deref().unwrap().borrow().left) + 1
}

fn get_right_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> u32 {
    if node.is_none() {
        return 0;
    }
    get_right_depth(&node.as_deref().unwrap().borrow().right) + 1
}

fn count_nodes(node: &Option<Rc<RefCell<TreeNode>>>, left_depth: Option<u32>, right_depth: Option<u32>) -> u32 {
    let left_depth = left_depth.unwrap_or(get_left_depth(node));
    let right_depth = right_depth.unwrap_or(get_right_depth(node));
    if left_depth == right_depth {
        return 2_u32.pow(left_depth as u32) - 1;
    }
    let ref_node = &node.as_deref().unwrap().borrow();
    let left_child = &ref_node.left;
    let left_child_left_depth = left_depth - 1;
    let left_child_right_depth = get_right_depth(left_child);
    let right_child_right_depth = right_depth - 1;
    if left_child_left_depth == left_child_right_depth {
        let right_child = &ref_node.right;
        2_u32.pow(left_child_left_depth) + count_nodes(right_child, None, Some(right_depth - 1))
    } else {
        count_nodes(left_child, Some(left_child_left_depth), Some(left_child_right_depth)) + 2_u32.pow(right_child_right_depth)
    }
}

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        count_nodes(&root, None, None) as i32
    }
}