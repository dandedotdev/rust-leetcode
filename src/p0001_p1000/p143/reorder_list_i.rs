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
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }
        // split the list into two halves
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }
        // reverse the second half
        let mid = len >> 1;
        let mut cur = head.as_mut().unwrap();
        for _ in 0..mid - 1 {
            cur = cur.next.as_mut().unwrap();
        }
        let second_half = cur.next.take();
        let reversed_second_half = ReverseLinkedListSolution::reverse_list(second_half);
        // merge the two halves
        let mut first = head.as_mut().unwrap();
        let mut second = reversed_second_half;
        while let Some(second_node) = second {
            let temp = first.next.take();
            first.next = Some(second_node);
            first = first.next.as_mut().unwrap();
            second = temp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }

    #[test]
    fn test_case_2() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        }));
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }
}
