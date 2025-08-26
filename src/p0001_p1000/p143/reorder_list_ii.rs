use crate::{
    p0001_p1000::p206::reverse_linked_list::Solution as ReverseLinkedListSolution,
    structs::singly_linked_list::ListNode,
};

// <Two Pointers, Linked List, Recursion, Stack>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let ptr = head.clone();
        let (mut fast, mut slow) = (ptr.as_ref(), head.as_mut());
        while let Some(node) = fast {
            if node.next.is_none() || node.next.as_ref().unwrap().next.is_none() {
                break;
            }
            fast = node.next.as_ref().unwrap().next.as_ref();
            slow = slow.unwrap().next.as_mut();
        }
        // reverse the second half
        let second_half = slow.unwrap().next.take();
        let reversed_second_half = ReverseLinkedListSolution::reverse_list(second_half);
        // merge the two halves
        let (mut first_half, mut second_half) = (head.as_mut().unwrap(), reversed_second_half);
        while let Some(node) = second_half {
            let temp = first_half.next.take();
            first_half.next = Some(node);
            first_half = first_half.next.as_mut().unwrap();
            second_half = temp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let mut head = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 4]);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1, 4, 2, 3]);
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }

    #[test]
    fn test_case_2() {
        let mut head = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1, 5, 2, 4, 3]);
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }
}
