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

fn fill_levels(node: &Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<i32>>, mut current_level: usize) {
    if node.is_none() {
        return;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    if levels.len() > current_level {
        levels[current_level].push(node_ref.val);
    } else {
        levels.push(vec![node_ref.val]);
    }
    current_level += 1;
    fill_levels(&node_ref.left, levels, current_level);
    fill_levels(&node_ref.right, levels, current_level);
}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        fill_levels(&root, &mut result, 0);
        for level in result.iter_mut().skip(1).step_by(2) {
            let end = level.len() - 1;
            for index in 0..=end / 2 {
                level.swap(index, end - index);
            }
        }
        result
    }
}