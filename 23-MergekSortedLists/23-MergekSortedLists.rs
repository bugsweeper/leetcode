// Last updated: 19.05.2025, 09:53:40
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
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct HeapNode {
    top: i32,
    list: Option<Box<ListNode>>,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.top.cmp(&self.top)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top
    }
}

impl Eq for HeapNode {}

impl TryFrom<Option<Box<ListNode>>> for HeapNode {
    type Error = ();
    fn try_from(value: Option<Box<ListNode>>) -> Result<Self, ()> {
        if value.is_none() {
            return Err(());
        }
        let ListNode { val, next } = *value.unwrap();
        Ok(Self {
            top: val,
            list: next,
        })
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = lists
            .into_iter()
            .filter_map(|list| HeapNode::try_from(list).ok())
            .collect::<BinaryHeap<_>>();
        let mut head = None;
        let mut node = &mut head;
        while let Some(least_head) = heap.pop() {
            *node = Some(Box::new(ListNode {
                val: least_head.top,
                next: None,
            }));
            node = &mut node.as_mut().unwrap().next;
            if let Ok(new_head) = HeapNode::try_from(least_head.list) {
                heap.push(new_head);
            }
        }
        head
    }
}