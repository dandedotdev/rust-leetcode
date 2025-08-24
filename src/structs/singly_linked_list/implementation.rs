use super::ListNode;

/// Self-defined traits
pub trait LinkedListExt {
    fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>>;
}

impl LinkedListExt for Option<Box<ListNode>> {
    fn from_vec(values: Vec<i32>) -> Self {
        if values.is_empty() {
            return None;
        }
        let mut head = None;
        let mut cur = &mut head;
        for &v in values.iter() {
            *cur = Some(Box::new(ListNode::new(v)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}
