use crate::structs::singly_linked_list::ListNode;

// <Linked List, Recursion>
// Time: O(n)
// Space: O(1)

// Note: Iteration

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut cur) = (None, head);
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 4, 5]);
        let result = Solution::reverse_list(head);
        let expected = Option::<Box<ListNode>>::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2]);
        let result = Solution::reverse_list(head);
        let expected = Option::<Box<ListNode>>::from_vec(vec![2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let head: Option<Box<ListNode>> = Option::<Box<ListNode>>::from_vec(Vec::new());
        let result = Solution::reverse_list(head);
        let expected: Option<Box<ListNode>> = Option::<Box<ListNode>>::from_vec(Vec::new());
        assert_eq!(result, expected);
    }
}
