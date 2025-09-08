use crate::{
    p0001_p1000::p206::reverse_linked_list::Solution as ReverseLinkedListSolution,
    structs::singly_linked_list::ListNode,
};

// <Linked List, Recursion>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        for _ in 0..k {
            if let Some(node) = cur {
                cur = &mut node.next;
            } else {
                return head;
            }
        }
        let ans = Self::reverse_k_group(cur.take(), k);
        let rev = ReverseLinkedListSolution::reverse_list(head);
        Self::append(rev, ans)
    }

    fn append(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => tail,
            Some(mut head_node) => {
                head_node.next = Self::append(head_node.next, tail);
                Some(head_node)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 4, 5]);
        let k = 2;
        let result = Solution::reverse_k_group(head, k);
        let expected = Option::<Box<ListNode>>::from_vec(vec![2, 1, 4, 3, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 4, 5]);
        let k = 3;
        let result = Solution::reverse_k_group(head, k);
        let expected = Option::<Box<ListNode>>::from_vec(vec![3, 2, 1, 4, 5]);
        assert_eq!(result, expected);
    }
}
