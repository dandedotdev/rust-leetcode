use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::structs::doubly_linked_list::ListNode;

// <Design, Doubly-Linked List, Hash Table, Linked List>
// Time:
//   - get: O(1)
//   - put: O(1)
// Space:
//   - O(capacity)

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let node = Rc::clone(node);
            self.remove(&node);
            self.push_front(&node);
            node.borrow().val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.map.get(&key) {
            Some(node) => {
                let node = Rc::clone(node);
                node.borrow_mut().val = value;
                self.remove(&node);
                self.push_front(&node);
            },
            None => {
                let node = Rc::new(RefCell::new(ListNode::new(key, value)));
                self.push_front(&node);
                self.map.insert(key, node);
                if self.map.len() > self.capacity
                    && let Some(node) = self.pop_back()
                {
                    self.map.remove(&node.borrow().key);
                }
            },
        }
    }

    fn pop_back(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        self.tail.take().inspect(|node| {
            self.remove(node);
        })
    }

    fn push_front(&mut self, node: &Rc<RefCell<ListNode>>) {
        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = Some(Rc::downgrade(node));
                node.borrow_mut().next = Some(Rc::clone(&head));
                node.borrow_mut().prev = None;
                self.head = Some(Rc::clone(node));
            },
            None => {
                self.head = Some(Rc::clone(node));
                self.tail = Some(Rc::clone(node));
            },
        }
    }

    fn remove(&mut self, node: &Rc<RefCell<ListNode>>) {
        match (
            node.clone().borrow().prev.as_ref(),
            node.clone().borrow().next.as_ref(),
        ) {
            (Some(prev), Some(next)) => {
                if let Some(prev) = prev.upgrade() {
                    prev.borrow_mut().next = Some(Rc::clone(next));
                }
                next.borrow_mut().prev = Some(prev.clone());
            },
            (Some(prev), None) =>
                if let Some(prev) = prev.upgrade() {
                    self.tail = Some(Rc::clone(&prev));
                    prev.borrow_mut().next = None;
                },
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head = Some(Rc::clone(next));
            },
            (None, None) => {
                self.head = None;
                self.tail = None;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
