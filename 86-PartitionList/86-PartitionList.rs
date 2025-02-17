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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut l_head = None;
        let mut ge_head = None;
        let l_ref = &mut &mut l_head;
        let ge_ref = &mut &mut ge_head;
        while head.is_some() {
            // select destination list
            if head.as_mut().unwrap().val < x {
                // Disconnect head from input list
                let mut second = head.as_mut().unwrap().next.take();
                // Move meaningful value from disconnected head to destination list
                std::mem::swap(*l_ref, &mut head);
                // move pointer to empty next field (tail of destination list)
                *l_ref = &mut l_ref.as_mut().unwrap().next;
                // move `head` to head of disconnected list
                std::mem::swap(&mut head, &mut second);
            } else {
                // Disconnect head from input list
                let mut second = head.as_mut().unwrap().next.take();
                // Move meaningful value from disconnected head to destination list
                std::mem::swap(*ge_ref, &mut head);
                // move pointer to empty next field (tail of destination list)
                *ge_ref = &mut ge_ref.as_mut().unwrap().next;
                // move `head` to head of disconnected list
                std::mem::swap(&mut head, &mut second);
            }
        }
        // reconnect two sublists
        std::mem::swap(*l_ref, &mut ge_head);
        l_head
    }
}