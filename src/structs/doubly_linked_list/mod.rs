use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// Definition for doubly-linked list.
#[derive(Debug)]
pub struct ListNode {
    pub key: i32,
    pub val: i32,
    pub prev: Option<Weak<RefCell<ListNode>>>,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    pub fn new(key: i32, val: i32) -> Self {
        ListNode {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}
