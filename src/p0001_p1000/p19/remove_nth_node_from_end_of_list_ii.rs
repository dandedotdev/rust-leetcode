use crate::structs::singly_linked_list::ListNode;

// <Linked List, Two Pointers>
// Time: O(n)
// Space: O(1)

// Idiomatic

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let count = std::iter::successors(head.as_ref(), |node| node.next.as_ref()).count();
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let prev =
            (0..count - n as usize).fold(dummy.as_mut(), |cur, _| cur.next.as_mut().unwrap());
        prev.next = prev.next.as_mut().unwrap().next.take();
        dummy.next
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
