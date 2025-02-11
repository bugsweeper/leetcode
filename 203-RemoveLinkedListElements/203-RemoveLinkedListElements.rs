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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let ref_node = &mut &mut head;
        while ref_node.is_some() {
            if ref_node.as_mut().unwrap().val == val {
                // Disconnect next node from current
                let mut node = ref_node.take();
                // Reconnect node after next node to current
                std::mem::swap(*ref_node, &mut node.as_mut().unwrap().next);
            } else {
                // Go to next node
                *ref_node = &mut ref_node.as_mut().unwrap().next;
            }
        }
        head
    }
}