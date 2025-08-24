use crate::structs::singly_linked_list::ListNode;

// <Linked List, Two Pointers>
// Time: O(n)
// Space: O(1)

// Iteration

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut len = 0;
        let mut cur = dummy.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }
        if len == n {
            return dummy.unwrap().next.unwrap().next;
        }
        let mut prev = &mut dummy;
        for _ in 0..(len - n - 1) {
            prev = &mut prev.as_mut().unwrap().next;
        }
        if let Some(node) = prev {
            node.next = node.next.as_mut().unwrap().next.take();
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 4, 5]);
        let n = 2;
        let result = Solution::remove_nth_from_end(head, n);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1]);
        let n = 1;
        let result = Solution::remove_nth_from_end(head, n);
        let expected = Option::<Box<ListNode>>::from_vec(Vec::new());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2]);
        let n = 1;
        let result = Solution::remove_nth_from_end(head, n);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1]);
        assert_eq!(result, expected);
    }
}
