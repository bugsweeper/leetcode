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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 || head.is_none() {
            return head;
        }
        let mut head = head;
        let ref_node = &mut &mut head;
        // Go to end once for defining list length
        let mut size = 0;
        while ref_node.is_some() {
            size += 1;
            *ref_node = &mut ref_node.as_mut().unwrap().next;
        }
        // Items between cut position and tail of list
        let cut2end = k % size;
        if cut2end == 0 {
            // Looks like list rotated to itself
            return head;
        }
        // Items between cut position and head of list
        let cut2head = size - cut2end;
        // Finding cut position
        let ref_node = &mut &mut head;
        for _ in 0..cut2head {
            *ref_node = &mut ref_node.as_mut().unwrap().next;
        }
        // Making cut
        let mut new_head = ref_node.take();
        // Finding tail
        let ref_node = &mut &mut new_head;
        while ref_node.is_some() {
            *ref_node = &mut ref_node.as_mut().unwrap().next;
        }
        // Connecting old tail to old head
        std::mem::swap(*ref_node, &mut head);
        new_head
    }
}