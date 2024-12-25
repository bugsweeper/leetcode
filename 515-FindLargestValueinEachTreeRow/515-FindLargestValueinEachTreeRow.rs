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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut levels_max = Vec::new();
        if let Some(root) = root {
            let mut stack = Vec::new();
            stack.push((root, 0));
            while let Some((node, level)) = stack.pop() {
                let TreeNode { val, left, right } = Rc::into_inner(node).unwrap().into_inner();
                if levels_max.len() == level {
                    levels_max.push(val);
                } else {
                    levels_max[level] = levels_max[level].max(val);
                }
                if let Some(right) = right {
                    stack.push((right, level + 1));
                }
                if let Some(left) = left {
                    stack.push((left, level + 1));
                }
            }
        }
        levels_max
    }
}