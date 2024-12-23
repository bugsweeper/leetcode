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
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            // Converting tree to vec of levels
            let mut stack = Vec::new();
            stack.push((root, 0));
            let mut levels = Vec::new();
            while let Some((node, level)) = stack.pop() {
                let TreeNode{val, left, right} = Rc::into_inner(node).unwrap().into_inner();
                if levels.len() == level {
                    levels.push(vec![val]);
                } else {
                    levels[level].push(val);
                }
                if let Some(right) = right {
                    stack.push((right, level + 1));
                }
                if let Some(left) = left {
                    stack.push((left, level + 1));
                }
            }
            let mut answer = 0;
            for level in &mut levels[1..] {
                let mut sorted_level = level.clone();
                sorted_level.sort_unstable();
                let mut indexes = std::collections::HashMap::with_capacity(level.len());
                indexes.extend(level.iter().enumerate().map(|(index, &value)| (value, index)));
                for i in 0..level.len() {
                    if level[i] != sorted_level[i] {
                        answer += 1;
                        let to = indexes[&sorted_level[i]];
                        level.swap(i, to);
                        *indexes.get_mut(&level[to]).unwrap() = to;
                    }
                }
            }
            answer
        } else {
            0
        }
    }
}