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
#[inline]
fn peek_smaller(list1: &mut Option<Box<ListNode>>, list2: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let value1 = list1.as_ref().unwrap().val;
    let value2 = list2.as_ref().unwrap().val;
    if value1 > value2 {
        let mut node = list2.as_mut().unwrap().next.take();
        std::mem::swap(&mut node, list2);
        node
    } else {
        let mut node = list1.as_mut().unwrap().next.take();
        std::mem::swap(&mut node, list1);
        node
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }
        let mut list1 = list1;
        let mut list2 = list2;
        let mut result = peek_smaller(&mut list1, &mut list2);
        let mut pointer = &mut result.as_mut().unwrap().next;
        while list1.is_some() && list2.is_some() {
            std::mem::swap(pointer, &mut peek_smaller(&mut list1, &mut list2));
            pointer = &mut pointer.as_mut().unwrap().next;
        }
        if list1.is_none() {
            std::mem::swap(pointer, &mut list2);
        } else {
            std::mem::swap(pointer, &mut list1);
        }
        result
    }
}