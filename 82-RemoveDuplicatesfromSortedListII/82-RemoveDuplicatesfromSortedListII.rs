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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let ref_node = &mut &mut head;
        // Go to end once for defining list length
        while ref_node.is_some() {
            let next_node = ref_node.as_mut().unwrap();
            let current_val = next_node.val;
            let ref_next_node = &mut &mut next_node.next;
            let mut duplicates = 0;
            while ref_next_node.is_some() && ref_next_node.as_mut().unwrap().val == current_val {
                duplicates += 1;
                *ref_next_node = &mut ref_next_node.as_mut().unwrap().next;
            }
            if duplicates > 0 {
                // Disconnect first not duplicate node
                let mut node = ref_next_node.take();
                // Reconnect ot current node
                std::mem::swap(*ref_node, &mut node);
            } else if ref_node.is_some() {
                *ref_node = &mut ref_node.as_mut().unwrap().next;
            }
        }
        head
    }
}