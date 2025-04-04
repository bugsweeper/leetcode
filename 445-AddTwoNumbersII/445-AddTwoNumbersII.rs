// Last updated: 04.04.2025, 13:28:25
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::borrow::Borrow;

fn list2vec(mut node: &Option<Box<ListNode>>, vec: &mut Vec<i32>) {
    while node.is_some() {
        let node_ref: &ListNode = node.as_ref().unwrap().borrow();
        vec.push(node_ref.val);
        node = &node_ref.next;
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v1 = Vec::with_capacity(100);
        list2vec(&l1, &mut v1);
        let mut v2 = Vec::with_capacity(100);
        list2vec(&l2, &mut v2);
        if v1.len() < v2.len() {
            std::mem::swap(&mut v1, &mut v2);
        }
        let mut carry = 0;
        let mut result = None;
        let size_diff = v1.len() - v2.len();
        for (d1, d2) in v1.into_iter().rev().zip(v2.into_iter().rev().chain(std::iter::repeat_n(0, size_diff))) {
            let sum = d1 + d2 + carry;
            carry = sum / 10;
            result = Some(Box::new(ListNode{ val: sum % 10, next: result }));
        }
        if carry > 0 {
            result = Some(Box::new(ListNode{ val: carry, next: result }));
        }
        result
    }
}