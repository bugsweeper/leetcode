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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let ref_node = &mut &mut head;
        // Go to end once for defining list length
        let mut size = 0;
        while ref_node.is_some() {
            size += 1;
            *ref_node = &mut ref_node.as_mut().unwrap().next;
        }
        // Go to target node
        let ref_node = &mut &mut head;
        for _ in 0..size - n {
            *ref_node = &mut ref_node.as_mut().unwrap().next;
        }
        // Disconnect next node from current
        let mut node = ref_node.take();
        // Reconnect node after next node to current
        std::mem::swap(*ref_node, &mut node.as_mut().unwrap().next);
        head
    }
}