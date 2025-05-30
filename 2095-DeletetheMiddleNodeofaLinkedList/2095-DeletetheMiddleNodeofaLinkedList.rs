// Last updated: 30.05.2025, 12:34:28
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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node_ref = &head;
        let mut count = 0;
        while node_ref.is_some() {
            count += 1;
            node_ref = &node_ref.as_ref().unwrap().next;
        }
        if count == 1 {
            return None;
        }
        count /= 2;
        let mut head = head;
        let mut node_ref = &mut head;
        while count > 0 {
            node_ref = &mut node_ref.as_mut().unwrap().next;
            count -= 1;
        }
        *node_ref = node_ref.as_mut().unwrap().next.take();
        head
    }
}