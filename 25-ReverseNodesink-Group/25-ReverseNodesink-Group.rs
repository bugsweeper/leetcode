// Last updated: 12.05.2025, 11:31:59
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
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let mut head = head;
        let mut reversed = None;
        let mut reversed_ref = &mut reversed;
        let mut stack = Vec::with_capacity(k);
        while head.is_some() {
            let mut node = head.as_mut().unwrap().next.take();
            std::mem::swap(&mut node, &mut head);
            stack.push(node);
            if stack.len() == k {
                while let Some(node) = stack.pop() {
                    *reversed_ref = node;
                    reversed_ref = &mut reversed_ref.as_mut().unwrap().next;
                }
            }
        }
        for node in stack {
            *reversed_ref = node;
            reversed_ref = &mut reversed_ref.as_mut().unwrap().next;
        }
        reversed
    }
}