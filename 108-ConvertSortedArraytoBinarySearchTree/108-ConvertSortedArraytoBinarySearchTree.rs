// Last updated: 01.05.2025, 23:18:38
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

fn sorted_slice_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        None
    } else {
        let middle = nums.len() / 2;
        let left = if middle == 0 {
            None
        } else {
            sorted_slice_to_bst(&nums[0..middle])
        };
        let right = if middle == nums.len() - 1 {
            None
        } else {
            sorted_slice_to_bst(&nums[middle + 1..])
        };
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[middle],
            left,
            right,
        })))
    }
}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        sorted_slice_to_bst(&nums[..])
    }
}