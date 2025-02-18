use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug, Default)]
struct LinkedListNode {
    key: i32,
    value: i32,
    newer: Option<Rc<RefCell<LinkedListNode>>>,
    older: Option<Rc<RefCell<LinkedListNode>>>,
}

#[derive(Default)]
struct LRUCache {
    head: Option<Rc<RefCell<LinkedListNode>>>,
    tail: Option<Rc<RefCell<LinkedListNode>>>,
    /// Key to node map for constant time access to node by key inside list
    map: HashMap<i32, Rc<RefCell<LinkedListNode>>>,
    capacity: usize,
}

fn move_to_head(
    head: &mut Option<Rc<RefCell<LinkedListNode>>>,
    tail: &mut Option<Rc<RefCell<LinkedListNode>>>,
    node_ref: &mut Rc<RefCell<LinkedListNode>>,
) {
    let mut borrowed = node_ref.borrow_mut();
    let LinkedListNode { newer, older, .. } = borrowed.deref_mut();
    match (older.is_some(), newer.is_some()) {
        // It is already head
        (_, false) => {}
        // It is tail
        (false, _) => {
            // Reconnect tail to next item, now newer contains old tail
            std::mem::swap(tail, newer);
            // Steal link to old tail from new tail and put it to old head
            let stollen_link = tail.as_mut().unwrap().borrow_mut().older.take();
            head.as_mut().unwrap().borrow_mut().newer = stollen_link;
            // Now moved node will point to old head and head will point to None
            std::mem::swap(older, head);
            // Reconnect old tail node to head
            std::mem::swap(head, newer);
        }
        // It is somewhere in between head and tail
        _ => {
            // Invalidate older link (move its value to newer node's link to older)
            std::mem::swap(older, &mut newer.as_mut().unwrap().borrow_mut().older);
            let mut newer_clone = newer.clone();
            // At this stage neighbors are pointing to each other, and newer_clone is pointing to node_ref
            std::mem::swap(
                &mut newer_clone,
                &mut newer
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .older
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .newer,
            );
            // Let's finally put it to head (older now points to old head)
            std::mem::swap(head, older);
            // New head have no newer nodes
            *newer = None;
            // Old head should point to new head
            std::mem::swap(&mut older.as_mut().unwrap().borrow_mut().newer, &mut newer_clone);
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            map: HashMap::with_capacity(capacity),
            capacity,
            ..Default::default()
        }
    }

    // Also updates recentness of usage
    fn get(&mut self, key: i32) -> i32 {
        let LRUCache {
            head, tail, map, ..
        } = self;
        if let Some(node_ref) = map.get_mut(&key) {
            let value = node_ref.borrow().value;
            move_to_head(head, tail, node_ref);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let LRUCache {
            head,
            tail,
            map,
            capacity,
        } = self;
        if let Some(node_ref) = map.get_mut(&key) {
            node_ref.borrow_mut().value = value;
            move_to_head(head, tail, node_ref);
        } else {
            if map.len() == *capacity {
                // Should remove old node before add
                // 1. Detaching the oldest node from list
                let mut node = tail.as_mut().unwrap().borrow_mut().newer.take();
                // 2. Point tail to newer node, while detached node should be in `node`
                std::mem::swap(tail, &mut node);
                // 3. Drop node from `map`
                map.remove(dbg!(&node.as_mut().unwrap().borrow().key));
                if tail.is_some() {
                    // 4. Node from list should be forgotten at the end of scope => drop last link to it
                    tail.as_mut().unwrap().borrow_mut().older = None;
                }
            }
            if map.is_empty() {
                // First node is head and tail at same time
                let node = Rc::new(RefCell::new(LinkedListNode {
                    key,
                    value,
                    ..Default::default()
                }));
                map.insert(key, node.clone());
                *head = Some(node.clone());
                *tail = Some(node);
            } else {
                // Otherwise connect to head as the most fresh
                let node = Rc::new(RefCell::new(LinkedListNode {
                    key,
                    value,
                    older: head.take(),
                    ..Default::default()
                }));
                let link_for_previous_head = node.clone();
                node.borrow_mut()
                    .older
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .newer
                    .replace(link_for_previous_head);
                map.insert(key, node.clone());
                // Reinitialize head (it is None after previous action) by node as new head.
                *head = Some(node);
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */